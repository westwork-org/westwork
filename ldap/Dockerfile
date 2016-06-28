FROM resin/rpi-raspbian:jessie

MAINTAINER Ross Schulman

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y slapd ldap-utils

VOLUME ["/var/lib/ldap"]

EXPOSE 389

# Avoid ERROR: invoke-rc.d: policy-rc.d denied execution of start.
RUN echo "#!/bin/sh\nexit 0" > /usr/sbin/policy-rc.d

ADD start.sh /usr/bin/start.sh

CMD ["start.sh"]
