FROM amd64/node:12-alpine

COPY package.json .
COPY yarn.lock .
COPY tsconfig.json .

COPY model.ts .
COPY components ./components
COPY design ./design
COPY pages ./pages
COPY public ./public
COPY utils ./utils
COPY hooks ./hooks
# TODO shouldn't have to copy this for build
COPY stories ./stories

RUN yarn
RUN yarn build

CMD yarn start
