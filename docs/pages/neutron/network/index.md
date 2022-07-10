---
layout: default
title: Network
parent: Neutron
has_children: true
---

## Neutron Network (N2)

Networking is a first class citizen for a neutron system. The main bottleneck for most apps nowadays is the network speed and how its processed. A lot of security issues come from the internet, more specifically malicious scripts downloaded accidentally or remote hacking, packet sniffers. Not to mention bad firewalls. You prob shouldnt be accepting any packet.

What OSes seem to do right is proper checking of programs downloaded from the internet. Signed apps and hashing. Also verified sites and developers. Attempting to download from a site that isnt HTTPS encrypted or properly verified in the last few months will raise an error.

Central to the quantii system is a blockchain like password system. The blockchain is stored on quantii cloud. In order to read from it, you need to have a single SHA-256 key. Which the user would prob have locally written somewhere. If possible, tell them to write it down somewhere. In case they dont have access to their devices anymore. Never share it. To anyone. Anyone.

Most of the time, like samsung keychain or apple keychain, you simply log into quantii cloud and you become verified. Once a blockchain is generated, it can only be destroyed through its key. It can be pseudo read through other devices that itself take the hashed key. So your other devices actually dont have "full" access. Or quantii cloud generates a lesser key, which isnt the original key. And only lets the original key edit itself and the blockchain in "destructive" ways. Lesser keys are able to add passwords to the chain or update old passwords, but not remove them. To remove them you need the original sha 256 key (OSK).

Its easy to remake the blockchain if your key is compromised. Just plug in your yubikey/quantii key again and remake the blockchain with a newly generated key. The problem is if the data is still online. So quantii cloud recognises compromises and gives you warnings.
