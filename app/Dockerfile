FROM node:20-alpine

COPY package.json .
COPY yarn.lock .
COPY tsconfig.json .

COPY constants.ts .
COPY model.ts .
COPY components ./components
COPY design ./design
COPY pages ./pages
COPY public ./public
COPY utils ./utils
COPY hooks ./hooks
COPY stories ./stories

RUN yarn
RUN yarn build

CMD yarn start
