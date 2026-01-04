# 🍲 Kermesse Solidaria (Rust + Yew)

> Una plataforma moderna, segura y eficiente para la gestión de kermesses benéficas, construida con el poder de Rust.

![Banner](https://img.shields.io/badge/Status-Active-success)
![Rust](https://img.shields.io/badge/Backend-Actix--Web-orange?logo=rust)
![Yew](https://img.shields.io/badge/Frontend-Yew-red?logo=rust)
![DB](https://img.shields.io/badge/Database-PostgreSQL-blue?logo=postgresql)

## 📋 Descripción

**Kermesse Solidaria** es una aplicación full-stack diseñada para facilitar la organización, promoción y venta de platos en eventos solidarios. Permite a los organizadores gestionar sus eventos y menús, mientras que ofrece a los visitantes una experiencia visual atractiva para explorar las causas benéficas y colaborar.

Esta solución demuestra el potencial de **Rust** en el desarrollo web moderno, utilizando un stack 100% Rust tanto en el servidor (Actix) como en el cliente (Yew/WASM).

## ✨ Características Principales

### 🌍 Vista Pública (Visitantes)
- **Catálogo de Eventos**: Exploración visual de kermesses activas con tarjetas informativas.
- **Detalle del Evento**: Información completa de la causa, incluyendo foto del beneficiario, historia y horarios.
- **Menú Digital**: Lista interactiva de platos disponibles con fotos, precios y stock en tiempo real.
- **Información de Contacto**: Lista de colaboradores y vendedores asociados para realizar pedidos directos (WhatsApp/Teléfono).

### 🛡️ Panel de Organizador (Autenticado)
- **Gestión de Kermesses**: Creación intuitiva de nuevos eventos con todos los detalles necesarios.
- **Gestión de Platos**: Herramientas para agregar y modificar el menú ofrecido en cada kermesse.
- **Registro de Ventas**: Sistema rápido para registrar pedidos y actualizar el inventario (Demo).
- **Seguridad**: Autenticación robusta basada en JWT y hashing seguro de contraseñas.

## 🛠️ Tecnologías

### Backend 🦀
- **Lenguaje**: [Rust](https://www.rust-lang.org/) (Seguridad de memoria y alto rendimiento)
- **Framework Web**: [Actix-web](https://actix.rs/) (Líder en benchmarks de velocidad)
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) (ORM Asíncrono y Dinámico para Rust)
- **Base de Datos**: PostgreSQL
- **Autenticación**: JSON Web Tokens (JWT) & Bcrypt

### Frontend 🎨
- **Framework**: [Yew](https://yew.rs/) (Componentes reactivos en Rust compilado a WebAssembly)
- **Estilos**: [TailwindCSS](https://tailwindcss.com/) (Diseño responsivo y estético)
- **Bundler**: [Trunk](https://trunkrs.dev/) (Empaquetado y gestión de assets WASM)

## 🚀 Instalación y Despliegue

Sigue estos pasos para levantar el proyecto en tu entorno local.

### Prerequisitos
- [Rust & Cargo](https://rustup.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [Docker](https://www.docker.com/) (Opcional, para levantar la BD rápidamente)

### 1. Clonar el repositorio
```bash
git clone https://github.com/nex23/KermesseSolidariaRUST.git
cd KermesseSolidariaRUST
```

### 2. Configurar Base de Datos
Si tienes Docker, puedes iniciar una instancia lista para usar:
```bash
docker-compose up -d
```
*Si no usas Docker, asegúrate de crear una base de datos PostgreSQL y configurar la variable `DATABASE_URL` en un archivo `.env`.*

### 3. Inicializar Datos (Semilla)
Aplica las migraciones y carga datos de prueba (Usuarios, Kermesses, Platos):

```bash
# Terminal 1: Preparación
# Instala sea-orm-cli si no lo tienes: cargo install sea-orm-cli

sea-orm-cli migrate up           # Crea las tablas
cargo run --bin backend -- --seed # Carga datos de ejemplo
```

### 4. Ejecutar Backend
```bash
# Terminal 1: Servidor API
cargo run --bin backend
```
El servidor escuchará en: `http://127.0.0.1:8080`

### 5. Ejecutar Frontend
```bash
# Terminal 2: Cliente Web
cd frontend
trunk serve --port 8000
```
La aplicación estará disponible en: `http://127.0.0.1:8000`

## 👤 Usuarios de Prueba

| Rol | Email | Password |
|-----|-------|----------|
| **Organizador** | `thenex@gmail.com` | `123456` |
| **Vendedor** | `vendor1@example.com` | `123456` |

## 🤝 Contribución
¡Las contribuciones son bienvenidas! Si tienes ideas para mejorar la plataforma, por favor abre un *issue* o envía un *pull request*.

## 📄 Licencia
Este proyecto es de código abierto.
