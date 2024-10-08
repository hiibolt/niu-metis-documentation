# 5. Advanced Metis Usage Techniques
One of the most powerful tricks on Metis is SSH automation.

This allows a Metis user to automate what would otherwise be:
* **1** - Logging into Metis over SSH
* **2** - Running job submission commands
* **3** - Retrieving a job ID

By doing this, we can intergrate Metis into the workflow of any existing web server!

This technique also opens the door to other techniques, three of which will be briefly mentioned in **Chapter 4.2 - Conceptual Techniques**. As the title states, because of the varied and complex nature of implementation they will only be described conceptually.

## Overview of the Chapters

### Chapter 5.1: SSH Automation with Metis
* **Goals**: Learn how to automate commands over SSH with Metis.
* **Problem**: Metis does not allow web servers, making automation difficult.
* **Solution**: Use an SSH library to open a multiplexed connection for execution.
* **Outcome**: You will be able to run any command on Metis programmatically.

### Chapter 5.2: Conceptual Techniques
* **Goals**: Learn how to further integrate Metis into your existing backend.
* **Problem**: Metis does not allow web servers.
* **Solution**: Use additional layers and API endpoints to proxy a backend.
* **Outcome**: You will be able to completely integrate your solution with Metis.