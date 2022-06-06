FROM node:16

# Set the Node environment to development to ensure all packages are installed
ENV NODE_ENV development

#install dependencies
WORKDIR /web

# Copy over `package.json` and lock files to optimize the build process
COPY ["package.json", "package-lock.json*", "npm-shrinkwrap.json*", "yarn.lock", "./"]
# Install Node modules
RUN yarn

# Copy all local files into image.
COPY . .

RUN yarn build


###
# Only copy over the Node pieces we need
# ~> Saves 35MB
###
FROM mhart/alpine-node:slim-12

WORKDIR /web

FROM nginx:1.18-alpine
RUN apk add yarn

COPY --from=0 /web .
COPY . .

EXPOSE 3000

# Run `yarn dev` and set the host to 0.0.0.0 so we can access the web app from outside
CMD ["yarn", "dev"]

# CMD ["node", "./build"]