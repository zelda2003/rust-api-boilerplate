# Rust API Boilerplate

A boilerplate for building scalable and production-ready REST APIs in Rust using **Actix-Web** and **SQLx** with PostgreSQL. This project demonstrates a modular structure with best practices for managing configurations, routes, database connections, and more.

---

## 🚀 Features

- **Actix-Web**: High-performance web framework.
- **SQLx**: Async, compile-time verified SQL queries.
- **PostgreSQL**: Persistent database support.
- **Environment Variables**: Configurable using `.env`.
- **Dockerized**: Docker support for the application and PostgreSQL.
- **Migrations**: Database migrations with `sqlx-cli`.
- **Structured Code**: Modular design for scalability.

---

## 🛠️ Project Structure

```plaintext
├── Cargo.toml            # Rust dependencies and project metadata
├── Dockerfile            # Dockerfile for building the app container
├── docker-compose.yml    # Docker Compose setup for API and database
├── migrations/           # SQL migration files for database schema
├── src/
│   ├── config.rs         # Environment variable management
│   ├── db.rs             # Database connection pool setup
│   ├── handlers/         # Request handlers
│   ├── models/           # Data models and DTOs
│   ├── routes/           # API route configurations
│   ├── services/         # Business logic and database operations
│   └── main.rs           # Application entry point
```

---

## 🐳 Running with Docker

### **1. Clone the Repository**
```bash
git clone https://github.com/your-username/rust-api-boilerplate.git
cd rust-api-boilerplate
```

### **2. Build and Run with Docker Compose**
Ensure Docker is installed and running, then execute:
```bash
docker-compose up --build
```

This command starts the API and a PostgreSQL database.

### **3. Access the API**
- **Base URL**: `http://127.0.0.1:8080`
- Example Endpoints:
    - `GET /hello`: Health check.
    - `GET /users`: Fetch all users.
    - `POST /users`: Create a new user.

---

## 🛠️ Running Locally

### **1. Prerequisites**
- Rust (latest stable version)
- PostgreSQL (installed and running)
- `sqlx-cli` for database migrations:
  ```bash
  cargo install sqlx-cli
  ```

### **2. Set Up the Environment**
Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://user:password@localhost:5432/rustdb
PORT=8080
```

### **3. Apply Migrations**
Run migrations to set up the database schema:
```bash
sqlx migrate run
```

### **4. Run the Application**
Start the server locally:
```bash
cargo run
```

Access the API at `http://127.0.0.1:8080`.

---

## 📖 API Endpoints

### **Health Check**
- **GET /hello**
    - **Description**: Simple endpoint to verify the API is working.
    - **Response**: `"Hello, World!"`

### **Users**
- **GET /users**
    - **Description**: Retrieve all users.
    - **Response**:
      ```json
      [
        {
          "id": "uuid",
          "name": "John Doe",
          "email": "john.doe@example.com"
        }
      ]
      ```

- **POST /users**
    - **Description**: Create a new user.
    - **Request Body**:
      ```json
      {
        "name": "Jane Doe",
        "email": "jane.doe@example.com"
      }
      ```
    - **Response**:
      ```json
      {
        "id": "uuid",
        "name": "Jane Doe",
        "email": "jane.doe@example.com"
      }
      ```

---

## 🛡️ Environment Variables

| Variable        | Default     | Description                          |
|------------------|-------------|--------------------------------------|
| `DATABASE_URL`   | None        | PostgreSQL connection string         |
| `PORT`           | `8080`      | Port where the API runs              |

---

## 🧪 Running Tests

To run tests (if implemented):
```bash
cargo test
```

---

## 📦 Deploying

1. **Build the Docker Image**:
   ```bash
   docker build -t rust-api-boilerplate .
   ```

2. **Run the Container**:
   ```bash
   docker run -d --env-file .env -p 8080:8080 rust-api-boilerplate
   ```

3. **Push to Docker Hub** (optional):
   ```bash
   docker tag rust-api-boilerplate your-username/rust-api-boilerplate:latest
   docker push your-username/rust-api-boilerplate:latest
   ```

---
