FROM armhf/alpine:3.3

MAINTAINER Ross Schulman <ross@rbs.io>

RUN apk update && apk add openldap

VOLUME ["/var/lib/ldap"]
VOLUME ["/etc/openldap"]

EXPOSE 389
EXPOSE 636

COPY conf/slapd.conf /etc/openldap/slapd.conf
RUN ["slaptest", "-f", "/etc/openldap/slapd.conf", "-F", "/etc/openldap/slapd.d"]
RUN ["chown", "-R", "ldap:ldap", "/etc/openldap"]

CMD ["slapd", "-h", "ldap:/// ldapi:///", "-g", "ldap", "-u", "ldap", "-d", "0"]
