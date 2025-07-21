Full-Stack Product Management Application
=========================================

This repository contains a full-stack application designed for managing product records. It features a high-performance RESTful API built with Rust and a responsive web frontend developed using Angular, styled with Tailwind CSS.

Table of Contents
-----------------

*   [Project Overview](https://www.google.com/search?q=#project-overview)
    
*   [Coding Challenge Requirements](https://www.google.com/search?q=#coding-challenge-requirements)
    
*   [Features](https://www.google.com/search?q=#features)
    
*   [Application Screenshots](https://www.google.com/search?q=#application-screenshots)
    
*   [Technologies Used](https://www.google.com/search?q=#technologies-used)
    
*   [Running the Application](https://www.google.com/search?q=#running-the-application)
    
    *   [Prerequisites](https://www.google.com/search?q=#prerequisites)
        
    *   [Cloning the Repository](https://www.google.com/search?q=#cloning-the-repository)
        
    *   [Backend Setup (Rust)](https://www.google.com/search?q=#backend-setup-rust)
        
    *   [Frontend Setup (Angular)](https://www.google.com/search?q=#frontend-setup-angular)
        
    
*   [Coding Challenge Specifics](https://www.google.com/search?q=#coding-challenge-specifics)
    
*   [Future Enhancements](https://www.google.com/search?q=#future-enhancements)
    
*   [License](https://www.google.com/search?q=#license)
    

Project Overview
----------------

The core purpose of this application is to provide a simple yet robust CRUD (Create, Read, Update, Delete) interface for product records. Users can add new products, view existing ones in a dynamic list, modify their details, and remove them from the database. The application is designed with a clear separation of concerns between the backend API and the frontend user interface.

Coding Challenge Requirements
-----------------------------

This project was developed as part of a Lead Developer / Full-Stack Developer Coding Challenge with the following requirements:

*   Build a simple API using Rust that can handle user authentication and basic CRUD operations for products.
    
*   Implement a simple frontend using Angular or Svelte that consumes this API.
    
*   Use Tailwind CSS for styling.
    
*   Ensure the solution is deployed on a server (e.g., Docker, Kubernetes) and share the live link or GitHub repository.
    

Features
--------

*   **Product Creation:** Add new products with details like name, price, description, and stock quantity.
    
*   **Product Listing:** View all existing products in a responsive dashboard layout.
    
*   **Product Update:** Modify details of existing products.
    
*   **Product Deletion:** Remove products from the database.
    
*   **Responsive & Mobile Friendly UI:** User interface adapts seamlessly to various screen sizes (mobile, tablet, desktop) ensuring a great user experience on any device.
    
*   **Alert Notifications:** Provides user feedback for successful operations or errors.
    
*   **Initial Challenge Modal:** An introductory modal explaining the project's purpose and technologies.
    
*   **SQL Injection Immunity:** The Rust backend is designed to be immune to classical SQL injection attacks through the use of prepared statements.
    

Application Screenshots
-----------------------

Here are some screenshots showcasing the application's user interface and key functionalities.

- **Initial Challenge Modal**  
  You will be shown a text which explains that this is a challenge and its purpose.

![Initial Challenge Modal](https://github.com/user-attachments/assets/f1fe5be2-893b-4400-91b9-4d307d15ee9e)

- **Main Page**  
  After pressing continue for the initial challenge modal, you will be able to look at the products.
![My Products](https://github.com/user-attachments/assets/c90e007e-896d-4bfa-a425-13b88f3cfdc6)
- **Create/Update Modal**  
  When pressing the + button or the update button you will enter another modal which will allow you to enter input for the product fields and create or update the product.
![Create/Update Modal](https://github.com/user-attachments/assets/01f99105-a48d-4887-b7b3-26cbd2b0e645)

_Note: The product names, prices, descriptions, and stock quantities shown in the screenshots are for development and demonstration purposes only and do not represent real-world product data._

Technologies Used
-----------------

### Backend (Rust API)

*   **Rust:** A high-performance, memory-safe programming language.
    
*   **Actix-web:** A powerful and fast web framework for Rust, used for handling HTTP requests and responses.
    
*   **SQLite:** A lightweight, file-based relational database used for data persistence.
    
*   **Rusqlite:** A Rust binding for SQLite, providing the interface to interact with the database.
    
*   **serde:** For serialization and deserialization of JSON data.
    
*   **thiserror & lazy\_static:** Rust crates for robust error handling and lazy initialization of global resources (like the database connection pool).
    
*   **actix-cors:** Middleware for Actix-web to enable Cross-Origin Resource Sharing, allowing the frontend to securely communicate with the backend from a different origin.
    

### Frontend (Angular Web App)

*   **Angular:** A popular TypeScript-based framework for building dynamic single-page applications.
    
*   **TypeScript:** A superset of JavaScript that adds static typing, improving code quality and maintainability.
    
*   **Tailwind CSS:** A utility-first CSS framework used for rapidly styling the user interface, ensuring responsiveness and a modern look.
    
*   **Angular HttpClient:** Angular's built-in module for making HTTP requests to the backend API.
    
*   **RxJS:** Used for reactive programming, handling asynchronous operations (like HTTP responses) with Observables.
    

Running the Application
---------------

Follow these instructions to set up and run the application locally.

### Prerequisites

Before you begin, ensure you have the following installed on your system:

*   **Node.js and npm:** Required for the Angular frontend.
    
    *   [Download Node.js](https://nodejs.org/) (includes npm)
    
*   **Rust and Cargo:** Required for the Rust backend.
    
    *   [Install Rust (rustup)](https://rustup.rs/)
        
*   **Git:** For cloning the repository.
    
    *   [Download Git](https://git-scm.com/downloads)
        

### Cloning the Repository

1.  git clone https://github.com/skyfterri/Tossit-Coding_Challenge.git  
2.  cd Tossit-Coding_Challenge -> navigates to this path

### Backend Setup (Rust)

1.  cd product\_api -> navigates to this path
    
2.  cargo build -> This will download dependencies and compile the backend application.
    
3.  cargo run -> The API server will start, typically listening on http://127.0.0.1:8080. It will create a products.db SQLite file in this directory if it doesn't already exist.
    

### Frontend Setup (Angular)

1.  Open a new terminal window.
2.  Copy path to Tossit-Coding_Challenge
3.  cd product-frontend -> navigate to this directory 
4.  npm install -> Installs the project-specific dependencies listed in your package.json file
5.  npm install -g @angular/cli -> Installs the Angular CLI globally on your system.
6.  Open product-frontend/src/app/product.service.ts and ensure the apiUrl points to your backend:

// If running both locally on the same machine:private apiUrl = 'http://localhost:8080/products'

// If backend is on a remote server (e.g., Ubuntu VM IP):// private apiUrl = 'http://YOUR\_UBUNTU\_SERVER\_IP:8080/products'

**Important:** If your backend is running on a different machine, replace localhost with its IP address. Also, ensure your Rust backend's actix-cors configuration allows requests from your frontend's origin (e.g., http://localhost:4200 for local Angular development).

8.  ng serve --open ->This will open the Angular application in your default web browser, usually at http://localhost:4200.
      
Coding Challenge Specifics
--------------------------

This section explains certain decisions made during development and deviations from the initial challenge requirements.

### Why No Full User Authentication

The initial challenge outlined user authentication. While the Rust backend is capable of handling authentication, a full implementation was not included to:

*   **Focus on Core Product CRUD:** Prioritize building and demonstrating the primary product management functionalities.
    
*   **Keep Scope Focused:** Avoid adding significant complexity that would detract from the main goal of a functional product API and UI.
    
*   **Foundation for Future Work:** The current structure provides a clear path for adding authentication in a future iteration.
    

### Why No Full Docker/Kubernetes Deployment

The challenge also requested deployment using Docker or Kubernetes. While a Dockerfile for the product-frontend is provided, a complete multi-service deployment setup (like Docker Compose or Kubernetes manifests) was not included. This was to:

*   **Focus on Core Application Logic:** Ensure the primary Rust API and Angular frontend were functional and integrated.
    
*   **Simplify Initial Delivery:** Provide a runnable application without requiring complex orchestration setup for evaluation.
    
*   **Starting Point for Deployment:** The provided Dockerfile enables easy containerization for future deployment efforts.
    

Future Enhancements
-------------------

The future of this application could involve evolving it into a more comprehensive, real-world e-commerce or inventory management system. Potential enhancements include:

*   **User Roles and Permissions:** Implement distinct user roles (e.g., Customer, Admin, Worker) with granular permissions.
    
    *   **Customer Features:** Allow users to browse products, add them to a cart, and complete purchase transactions, which would automatically decrement product stock.
        
    *   **Admin/Worker Features:** Provide a dedicated interface for administrative tasks, enabling authorized personnel to:
        
        *   Create, update, and delete products.
            
        *   Manage product stock levels.
            
        *   View sales reports or order history.
            
*   **Order Management System:** Develop functionality to track customer orders, order status, and shipping details.
    
*   **Payment Gateway Integration:** Integrate with a third-party payment provider for secure online transactions.
    
*   **Advanced Search and Filtering:** Implement more sophisticated search capabilities, filtering by categories, price ranges, and other attributes.
    
*   **Improved Error Handling:** More granular error messages and a dedicated notification service in the frontend.
    
*   **Database Migrations:** Use a Rust crate for database migrations to manage schema changes more robustly.
    
*   **Testing:** Add comprehensive unit and integration tests for both frontend and backend.
    
*   **Advanced UI/UX:** Implement features like product image uploads, user reviews, and a more dynamic shopping experience.
    
*   **Full Docker Compose:** Provide a docker-compose.yml at the root to easily spin up both backend and frontend services in containers.
    

License
-------

This project is open-source and licensed under the MIT License. You are free to use, modify, and distribute this code
