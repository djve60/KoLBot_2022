Manual for KolBot_2022
======================
[Basing this on https://github.com/rlbond86/cwbot/tree/master/documentation]

1. Installation
---------------

KoLBot_2022 is a binary (i.e. compiled) coded in rust. 

To run a bot copy the files in <TBA> to where you want to run the code after updating initialization.ini.

Building and compiling instructions are in ....

2. Account Requirements
-----------------------

The code requires a dedicated KoL account to run. You must know the username and password of the account. 

Place account information in initialization.ini, which uses the "ini" format.

3. Running as a Service
-----------------------

If you want to run at startup, you should set it up as a service.

3.1 On Unix/Linux
-----------------
If you are using a unix variant, configuration files for init.d,  systemd and upstart are included in the dirctory …/documentation/unix. Simply copy this into the appropriate directory and edit the path parameters within to match your install location and user account. For init.d based systems don't forget to symlink the file into rc6.d directory.

You will need to edit the path to the binary. If your Linux distribution does not support initd, systemd, or upstart you will have to creat your own script.

Then reboot, or use "sudo service kolbot start" (or whatever is appropriae), to start the service. 

3.2 On Windows
--------------

NOTE: Windows has not been tested but should be close to steps listed below.

If you are using Windows, you should install this as a service. Do the following steps:

1. Find the install_service.bat file in the …/document/windows folder. 
   Right click it and select Run as administrator to install the service.
   
3. Open the services panel, which can be accessed by navigating:
   Control Panel > Administrative Tools > Services.
   
4. Find the service in the list and double-click it to open the service 
   properties. 
   
5. Change the startup drop-down to "Automatic". Then click the Start button 
   and then click OK when the service starts.
   
To remove the service, right-click the remove_service.bat file and click
Run as administrator. If you want to move the cwbot files to a different
folder, make sure to remove the service first.

If you want multiple instances, each running as a service, you will need
to install a separate copy of the code into different folders.



