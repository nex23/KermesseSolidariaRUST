use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;

use crate::entity::{users, kermesses, dishes, collaborators};
use crate::utils::hash;

pub async fn seed_db(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seeding database...");

    // 1. Create Users
    let users_data = vec![
        ("thenex", "thenex@gmail.com", "123456", "The Nex", "77777777"),
        ("vendor1", "vendor1@example.com", "123456", "Doña Julia", "60000001"),
        ("vendor2", "vendor2@example.com", "123456", "Don Carlos", "60000002"),
    ];

    for (username, email, password, full_name, phone) in users_data {
        if users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(conn)
            .await?
            .is_none()
        {
            let password_hash = hash::hash_password(password)?;
            users::ActiveModel {
                username: Set(username.to_string()),
                email: Set(email.to_string()),
                password_hash: Set(password_hash),
                full_name: Set(full_name.to_string()),
                phone: Set(phone.to_string()),
                created_at: Set(Utc::now().into()),
                ..Default::default()
            }
            .insert(conn)
            .await?;
            println!("Created user: {}", username);
        }
    }

    let organizer = users::Entity::find()
        .filter(users::Column::Username.eq("thenex"))
        .one(conn)
        .await?
        .expect("Organizer not found");

    let vendor1 = users::Entity::find()
        .filter(users::Column::Username.eq("vendor1"))
        .one(conn)
        .await?
        .expect("Vendor1 not found");

    let vendor2 = users::Entity::find()
        .filter(users::Column::Username.eq("vendor2"))
        .one(conn)
        .await?
        .expect("Vendor2 not found");


    // 2. Create Kermesses
    let kermesses_data = vec![
        (
            "Gran Kermesse por la Salud de la Tía María",
            "gran-kermesse-tia-maria",
            "Evento solidario para recaudar fondos para la cirugía de cadera de nuestra querida Tía María. ¡Habrá música en vivo y comida deliciosa!",
            "2026-02-15",
            "María Pérez",
            "Cirugía de cadera urgente",
            Some("https://images.unsplash.com/photo-1544005313-94ddf0286df2?q=80&w=200&auto=format&fit=crop".to_string()),
            Some("10:00".to_string()),
            Some("16:00".to_string()),
            "ACTIVE"
        ),
        (
            "Kermesse de los Bomberos Voluntarios",
            "kermesse-bomberos",
            "Ayúdanos a equipar a nuestros héroes. Todo lo recaudado será para comprar nuevos trajes y equipos de rescate.",
            "2026-03-01",
            "Cuerpo de Bomberos",
            "Adquisición de equipamiento de protección personal",
            Some("https://images.unsplash.com/photo-1554769062-8e1d51372c3d?q=80&w=200&auto=format&fit=crop".to_string()),
            Some("09:00".to_string()),
            Some("18:00".to_string()),
            "ACTIVE"
        ),
    ];

    for (name, slug, desc, date, b_name, b_reason, b_img, start, end, status) in kermesses_data {
        if kermesses::Entity::find()
            .filter(kermesses::Column::Slug.eq(slug))
            .one(conn)
            .await?
            .is_none()
        {
            let kermesse = kermesses::ActiveModel {
                name: Set(name.to_string()),
                slug: Set(slug.to_string()),
                description: Set(desc.to_string()),
                event_date: Set(NaiveDate::parse_from_str(date, "%Y-%m-%d")?),
                organizer_id: Set(organizer.id),
                beneficiary_name: Set(b_name.to_string()),
                beneficiary_reason: Set(b_reason.to_string()),
                beneficiary_image_url: Set(b_img),
                start_time: Set(start),
                end_time: Set(end),
                status: Set(status.to_string()),
                created_at: Set(Utc::now().into()),
                ..Default::default()
            }
            .insert(conn)
            .await?;
            println!("Created kermesse: {}", name);
            
            // Add dishes for this kermesse
            if slug == "gran-kermesse-tia-maria" {
                let dishes: Vec<(&str, &str, Decimal, i32, Option<String>)> = vec![
                    ("Picante de Pollo", "Delicioso picante de pollo con arroz, chuño y ensalada. Receta de la abuela.", Decimal::new(3500, 2), 50, Some("https://images.unsplash.com/photo-1569058242253-92a9c755a2c6?q=80&w=300&auto=format&fit=crop".to_string())),
                    ("Saice Chapaco", "Tradicional saice tarijeño servido con fideo y ensalada.", Decimal::new(3000, 2), 40, Some("https://images.unsplash.com/photo-1588166524941-2d3a3f5a5c6b?q=80&w=300&auto=format&fit=crop".to_string())),
                    ("Ranga Ranga", "Plato típico y picante para los amantes de la buena comida.", Decimal::new(2500, 2), 30, Some("https://images.unsplash.com/photo-1579366948929-444fc1f746dc?q=80&w=300&auto=format&fit=crop".to_string())),
                ];
                for (d_name, d_desc, price, qty, img) in dishes {
                    dishes::ActiveModel {
                       kermesse_id: Set(kermesse.id),
                       name: Set(d_name.to_string()),
                       description: Set(d_desc.to_string()),
                       price: Set(price),
                       quantity_available: Set(qty),
                       image_url: Set(img),
                       ..Default::default()
                    }.insert(conn).await?;
                }
                println!("Added dishes to {}", name);

                // Add collaborators
                collaborators::ActiveModel {
                    kermesse_id: Set(kermesse.id),
                    user_id: Set(vendor1.id),
                    role: Set("SELLER".to_string()),
                    ..Default::default()
                }.insert(conn).await?;
                 collaborators::ActiveModel {
                    kermesse_id: Set(kermesse.id),
                    user_id: Set(vendor2.id),
                    role: Set("SELLER".to_string()),
                    ..Default::default()
                }.insert(conn).await?;
                println!("Added collaborators to {}", name);

            } else if slug == "kermesse-bomberos" {
                let dishes: Vec<(&str, &str, Decimal, i32, Option<String>)> = vec![
                    ("Fricasé Paceño", "Exquisito fricasé de cerdo con mucho mote y chuño. Ideal para levantar el ánimo.", Decimal::new(4000, 2), 60, Some("https://images.unsplash.com/photo-1514326640560-7d063ef2aed5?q=80&w=300&auto=format&fit=crop".to_string())),
                    ("Chicharrón", "Cerdo frito crocante acompañado de mote, papa y llajua.", Decimal::new(4000, 2), 50, Some("https://images.unsplash.com/photo-1626082927389-6cd097cdc6ec?q=80&w=300&auto=format&fit=crop".to_string())),
                    ("Plato Paceño", "Clásico plato con queso frito, choclo, habas y papa.", Decimal::new(3500, 2), 40, Some("https://images.unsplash.com/photo-1534422298391-e4f8c172dddb?q=80&w=300&auto=format&fit=crop".to_string())),
                ];
                for (d_name, d_desc, price, qty, img) in dishes {
                    dishes::ActiveModel {
                       kermesse_id: Set(kermesse.id),
                       name: Set(d_name.to_string()),
                       description: Set(d_desc.to_string()),
                       price: Set(price),
                       quantity_available: Set(qty),
                       image_url: Set(img),
                       ..Default::default()
                    }.insert(conn).await?;
                }
                println!("Added dishes to {}", name);
                 // Add collaborators
                 collaborators::ActiveModel {
                    kermesse_id: Set(kermesse.id),
                    user_id: Set(organizer.id), // The organizer is also selling
                    role: Set("SELLER".to_string()),
                    ..Default::default()
                }.insert(conn).await?;
                println!("Added collaborators to {}", name);
            }
        }
    }

    println!("✅ Database seeding completed successfully!");
    Ok(())
}
