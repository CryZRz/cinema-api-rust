# Cinema Management REST API

REST API for cinema management developed in Rust, Axum, SeaORM, and PostgreSQL. It includes OpenAPI documentation via Swagger UI, containerization with Docker Compose, and Kubernetes deployment manifests.

## Features

- Movie catalog management: create, read, update, delete, and track duration, genre, and synopsis.
- Room and cinema hall management: track room names, seating capacities, and room types.
- Staff and employee registration: manage staff profiles, contact information, and roles.
- Screening scheduling: schedule movies in specific rooms with start and end times, ticket pricing, and seat tracking.
- Attendee bookings: reserve tickets, validate real-time seat availability, inspect attendees per screening, and handle booking cancellations with automatic seat restoration.
- Interactive OpenAPI / Swagger UI: test all endpoints directly through the web interface.
- Automatic database schema initialization: creates all required tables and relationships on startup.

## Architecture

- Language: Rust 2021 edition
- Web Framework: Axum 0.8
- Async Runtime: Tokio
- ORM and Query Engine: SeaORM 1.1 with PostgreSQL driver
- Documentation: utoipa and utoipa-swagger-ui
- Database: PostgreSQL 16
- Containerization: Docker and Docker Compose
- Orchestration: Kubernetes manifests with Kustomize

## Prerequisites

- Rust toolchain 1.85 or newer
- Cargo
- Docker and Docker Compose
- PostgreSQL 16 (if running locally without Docker)

## Configuration

The application is configured using environment variables or a local .env file:

- DATABASE_URL: PostgreSQL connection string (default: postgres://cinema_user:cinema_password@localhost:5433/cinema_db)
- SERVER_HOST: Host interface to bind the HTTP server (default: 0.0.0.0)
- SERVER_PORT: Port number to bind the HTTP server (default: 8080)
- RUST_LOG: Logging level (default: cinema_api_rust=info,tower_http=debug)

## Running with Docker Compose

Run the entire stack including PostgreSQL and the API:

```bash
docker compose up --build -d
```

View container logs:

```bash
docker compose logs -f
```

Stop the environment:

```bash
docker compose down
```

The database port is exposed at localhost:5433 to avoid conflicts with other local PostgreSQL instances. The API is accessible at http://localhost:8080.

## Running Locally

1. Start PostgreSQL:

```bash
docker compose up cinema-db -d
```

2. Export the database connection:

```bash
export DATABASE_URL="postgres://cinema_user:cinema_password@localhost:5433/cinema_db"
export SERVER_HOST="127.0.0.1"
export SERVER_PORT="8080"
```

3. Run the application:

```bash
cargo run
```

## Interactive Swagger UI

Open your browser and navigate to:

```
http://localhost:8080/swagger-ui
```

The raw OpenAPI JSON schema is available at:

```
http://localhost:8080/api-docs/openapi.json
```

## API Endpoints

### System

- GET /health: Health check status

### Movies

- GET /api/movies: List all movies
- POST /api/movies: Create a new movie
- GET /api/movies/{id}: Retrieve a movie by ID
- PUT /api/movies/{id}: Update a movie by ID
- DELETE /api/movies/{id}: Delete a movie by ID

### Rooms

- GET /api/rooms: List all cinema rooms
- POST /api/rooms: Create a cinema room
- GET /api/rooms/{id}: Retrieve a cinema room by ID
- PUT /api/rooms/{id}: Update a cinema room by ID
- DELETE /api/rooms/{id}: Delete a cinema room by ID

### Employees

- GET /api/employees: List all employees
- POST /api/employees: Register a new employee
- GET /api/employees/{id}: Retrieve an employee by ID
- PUT /api/employees/{id}: Update an employee by ID
- DELETE /api/employees/{id}: Delete an employee by ID

### Screenings

- GET /api/screenings: List all scheduled screenings
- POST /api/screenings: Schedule a screening for a movie and room
- GET /api/screenings/{id}: Retrieve screening details
- PUT /api/screenings/{id}: Update a screening
- DELETE /api/screenings/{id}: Cancel and delete a screening
- GET /api/screenings/{id}/attendees: List attendees for a screening

### Bookings

- GET /api/bookings: List all bookings
- POST /api/bookings: Book seats for a screening
- GET /api/bookings/{id}: Retrieve booking details
- POST /api/bookings/{id}/cancel: Cancel a booking and restore available seats

### Statistics

- GET /api/stats: Cinema operational and box office statistics

## Kubernetes Deployment

Deploy all resources to a Kubernetes cluster using Kustomize:

```bash
kubectl apply -k k8s/
```

Verify deployment status:

```bash
kubectl get all -n cinema-system
```

Remove deployment:

```bash
kubectl delete -k k8s/
```
