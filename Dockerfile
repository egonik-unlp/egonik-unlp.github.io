# Stage 1: Build the Leptos application
FROM rust:latest AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
# Install Trunk and build the Leptos CSR app
RUN cargo install --locked trunk

COPY . .
# Stage 2: Serve the built application with Nginx
RUN  trunk build --release


FROM nginx:alpine
# Copy the built assets from the builder stage
COPY --from=builder /app/dist /usr/share/nginx/html
# Expose port 80 for the web server
EXPOSE 80
# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
