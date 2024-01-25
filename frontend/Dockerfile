FROM node:18-alpine

WORKDIR /src

COPY package*.json ./

RUN npm install

COPY . .

RUN npm run build

RUN npm prune --production

ENV PORT=8080

EXPOSE 8080

CMD ["npm", "run", "start"]