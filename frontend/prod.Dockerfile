# Stage 1: Build the project
FROM node:18 AS build

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json to the container
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy the rest of the application code to the container
COPY . .

# Build the SvelteKit project
RUN npm run build

# Stage 2: Run the production build
FROM node:18 AS production

# Set environment variable for production
ENV NODE_ENV=production

# Set the working directory
WORKDIR /app

# Copy the package.json and package-lock.json files again
COPY package.json package-lock.json ./

# Install only production dependencies
RUN npm ci --only=production

# Copy the build output from the previous stage
COPY --from=build /app/build ./build
COPY --from=build /app/static ./static
COPY --from=build /app/src ./src

# Expose the port the application runs on
EXPOSE 3000

# Start the SvelteKit production server
CMD ["node", "./build"]
