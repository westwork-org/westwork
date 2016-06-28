FROM resin/rpi-raspbian:jessie

MAINTAINER Ross Schulman

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y slapd ldap-utils

VOLUME ["/var/lib/ldap"]

EXPOSE 389

ADD start.sh /usr/bin/start.sh

CMD ["start.sh"]
