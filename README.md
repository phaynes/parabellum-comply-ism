# Parabellum Comply (ISM)

Military / Government applications normally require an Authority to Operate (ATO) before they can be deployed into production. 
Within the Australian context, the Australian Cyber Security Centre (ACSC) publish the Information Security Manual (ISM) that represents 
the minimum set of control that applications must comply with.

The historical methodology for performing assessment, typically uses Excel spreadsheets, Word documents, and manual validation and allocation. An expensive
process that is error-prone, slow, and expensive. Making it difficult to repeat - even if you can get the ncessary classified staff.

Trouble is:
o There are many hundreds of controls that need to be allocated against major system and the controls 
o Controls change every 3 months in response to the changing cyber warfare landscape.
o The systems are highly complex and large
o Multiple environments such as Dev / Test / Acceptance / pre-prod and production must all be supported.
o The required rate of system change is high
o The rate of cyber risk evolution in contested cyberspace landscape is high.

in practice mannual approaches are not fit for use, except for all but the most tivial of systems.

This issue is well understood to the point that the US standards body NIST publish the [OSCAL] (https://pages.nist.gov/OSCAL/) as a standard for control automation. The ACSC publish the ISM in an OSCAL format.




This application aims to mitigate these issues by utilizing a URL-based system with the ISM OSCAL definition. This allows for direct allocation of requirements to specific components, integrating control definitions into the system to establish a foundational reference for allocation.


https://www.cyber.gov.au/resources-business-and-government/essential-cyber-security/ism

To build a set of VM's run the command 

```
Code goes here
```
