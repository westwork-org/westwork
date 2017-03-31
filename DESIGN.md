Four separate pieces to the puzzle.
1. Backend. In rust, manages all of the apps running in the background and passes messages between backend and user via websockets.
2. Front end. In React. Serves up the information to the user that it gets from the backend.
3. Gen. Builds the Rasberry Pi image up to where it can be burned on the SD card.
4. Config. Set up to run once on first boot to get the system where it needs to be for the user. Register domain name, figure out firewall situation, etc.

Front end API needs:
* Email queries (JMAP?)
* IM queries (matrix-react-sdk)
* Calender queries
* File sync queries (including photo browser)
