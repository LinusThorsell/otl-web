# Base image
FROM node:18

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json first to cache dependencies
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy all the application code to the container
COPY . .

# Build the SvelteKit project
RUN npm run build

# Expose the port your app will run on
EXPOSE 3000

CMD ["node", "/app/build"]
