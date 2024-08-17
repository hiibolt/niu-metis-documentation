# 4. Advanced Metis Usage Techniques
The first and only advanced technique this book will cover with examples is SSH automation.

This allows a Metis user to automate what would otherwise be:
* **1** - Logging into Metis over SSH
* **2** - Running job submission commands
* **3** - Retrieving a job ID

By doing this, we can intergrate Metis into the workflow of any existing web server!

This technique also opens the door to other techniques, three of which I will briefly mention below. Because of the varied and complex nature of implementation, they will only be described conceptually.

### Providing Files to Metis Remotely
By adding file IDs, download links, or using another way to communicate a download location, you can use the arguments on a job submission request to provide Metis with a way to download files for processing.

This can be accomplished by reading the provided arguments in your PBS script, and using `wget`, `git`, `curl`, Git LFS, or another download tool to then download the files onto Metis and into the PBS job's working directory.

### Web Server Completion Reporting
Since PBS jobs on Metis have the ability to connect to the internet, it's possible to then ping your webserver to let it know it's finished, instead of guessing.

The process can look like:
- Create a database to track jobs on your webserver
- Create a route that allows updating each job entry via HTTP
- Create a new job data structure in your database with a unique ID for a job
- Pass the unique ID to the SSH automation as an argument when submitting a new job
- Recieve and note that argument in your PBS script file
- When work in your PBS script file is done, at the very end, send an HTTP request to the updating route
- Update the database entry via the route, and handle any interpretation logic for the results of your job

This means your server can be aware of the moment your job is complete, and accomplish interpretation results immediately.

Due to the complex and implementation-specific nature of this process, I have not included an example. However, this technique was implemented in our backend for the iGait project, the link to which can be found [here](https://github.com/igait-niu/igait-backend)!

### Event Reporting Websocket
This technique only applies to jobs which are short enough to be tracked throughout the lifecycle of a single websocket connection, but can provide real-time results nonetheless.

The steps are mildly similar to the previous technique:
- Create an (asynchronus and thread-safe) websocket-compatible route, that when opened, first broadcasts a 'starting' event
- Create a route that allows updating each job entry via HTTP
- Create a new job data structure in your database with a unique ID for a job
- Pass the unique ID to the SSH automation as an argument when submitting a new job
- Recieve and note that argument in your PBS script file
- At each step, send an HTTP request to the webserver with any events you would like to broadcast
- At each invocation on the HTTP route, grab a handle to the websocket the ID corresponds to, and broadcast the information from the HTTP request
- When the job provides a completion signal, or when you send a fatal error event from your PBS script, close the websocket

This is more effective for jobs that may not have a 'final output', but rather, work in chunks. Two common examples are audio encoding/decoding, or token-by-token output from a machine learning model.
