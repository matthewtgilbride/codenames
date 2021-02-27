FROM amd64/node:12-alpine

COPY package.json .
COPY yarn.lock .
COPY .babelrc .
COPY tsconfig.json .

COPY model.ts .
COPY components ./components
COPY design ./design
COPY pages ./pages
COPY public ./public
COPY utils ./utils

RUN yarn
RUN yarn build

CMD yarn start
