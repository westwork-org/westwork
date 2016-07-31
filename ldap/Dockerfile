FROM armhf/alpine:3.3

MAINTAINER Ross Schulman <ross@rbs.io>

RUN apk update && apk add openldap

VOLUME ["/var/lib/ldap"]
VOLUME ["/etc/openldap"]

EXPOSE 389
EXPOSE 636

COPY conf/config.ldif /etc/openldap/config.ldif
RUN ["slapadd", "-F", "/etc/openldap/slapd.d", "-l", "/etc/openldap/config.ldif"]
RUN ["chown", "-R", "ldap:ldap", "/etc/openldap"]

CMD ["slapd", "-h", "ldap:/// ldapi:///", "-g", "ldap", "-u", "ldap", "-d", "0"]
