# o from diz para o docker que imagem baixar e que versão baixar
# após os dois pontos fica especificado a versão da imagem
FROM ubuntu:22.04
RUN apt update
WORKDIR /usr/src/app
RUN rm -r /usr/src/app
COPY /artefacts .
COPY lunes-staging-raw.json .
EXPOSE 9944
