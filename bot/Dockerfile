FROM node:18

WORKDIR /usr/src/app

COPY . .
RUN npm install
ENV CLIENT_TOKEN="OTc1OTU2MzgyNDg0NDAyMTc3.Gn2FGv.mYVGUmghQEJ_J2EVE_RwoPgLycNmTDbxWo0bmI"
ENV API_SERVER="http://localhost"
RUN npm run build
CMD [ "npm", "run", "start" ]
EXPOSE 3000