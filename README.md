Westwork
===

Westwork is a platform for a personal cloud that aims to replicate many if not all of the services that an average person would expect to receive from an online service provider. These services include, but are not limited to, mail, calendaring, instant messaging, file storage and syncing, and social networking. The system intends to be as simple as possible to install, instantiate, and operate. It should also be  secure against the greatest possible range of attacks and protective of the privacy of its owner. Finally, down the road, it should aim to be an intelligent assistant that could give 100% personalized suggestions and responses.

Purpose
---
In the past, the art of maintaining a secure, resilient, and robust online server was one that was mastered by a select few. Most average users who had an account of any sort held only email accounts (this was before the age of online calendars or data storage). If they were in university, their school provided the account. Otherwise, people were left trusting their personal email to their Internet Service Provider or an online web-based service like Yahoo! Mail.

As a consequence, we have become used to other entities holding and controlling our data. Not just our email anymore, but indeed all of our digital lives. Calendars, contact books, photos, personal documents... The list of items which we would never in a prior age have deposited with a third party but do today goes on.

The purpose of Westwork is to reverse this trend. The public should have options. The option to have an inexpensive, easy to use, secure, standards-based, and interoperable place to keep their digital lives. The digital file drawer in the home office.

Engineering Vision
---
Westwork will be composed of:
* A suite of battle-tested independent services designed to run on a Raspberry Pi (or equivalent), each doing one thing and interacting with one another, e.g.
    * Nginx for web serving
    * Postfix for mail receipt
    * LDAP for user management
    * Owncloud for data syncing and storage
    * Matrix for chat
    * etc
* A web app (and apps for iOS and Android) that provides both access to mail, documents, and the rest of the stored data, as well as administrative control over the system
    * The UI of this element should be paramount. It should be as easy to use as Gmail, as good looking as anything designed by Apple
    * It should unify the diverse independent services running underneath it into a coherent whole
* (Eventually) An artificially intelligent assistant capable of interacting with and learning from the independent services that make up Westwork

Current Steps
---
* Minimum Viable Product right now is:
    * Nginx
    * Postfix
    * IMAP Server (probably Cyrus)
    * Roundcube
    * LDAP
    * Owncloud (maybe)
    * Let's Encrypt for certs
* Create Raspberry Pi image generator (see the [westwork-gen](https://github.com/westwork-org/westwork-gen) repository)
* Create configuration files for the MVP services, in template form
    * Security is one of our most vital priorities, so these configs should err on the side of caution
* Figure out how to spin up the server for the first time and fill out configuration files (see the [westwork-bootstrap](https://github.com/westwork-org/westwork-bootstrap) repository)
* Some simple configuration panel that can administer - e.g. add users
