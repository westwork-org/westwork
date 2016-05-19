FROM resin/rpi-raspbian:jessie

MAINTAINER Ross Schulman

RUN apt-get update && apt-get install -y nginx

VOLUME ["/etc/nginx"]
VOLUME ["/usr/share/nginx/www"]

EXPOSE 80 443

CMD ["nginx"]
