FROM resin/rpi-raspbian:jessie

MAINTAINER Ross Schulman

RUN apt-get update && \
	apt-get install -y slapd ldap-utils

VOLUME ["/etc/ldap"]

EXPOSE 389

CMD ["slapd"]
