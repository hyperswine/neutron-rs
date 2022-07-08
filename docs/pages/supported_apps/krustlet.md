---
layout: default
title: Krustlet
parent: Supported Software
---

## Kubernetes

Kubernetes is a software program that deploys a cluster.

So when you deploy Kubernetes, you get a single luster on that machine.

Each cluster is composed of worker nodes. Each node can run its own containerised app. Like a docker container.

The worker nodes also host 'Pods'. Pods are components of the node's application workload.

- to manage/interface with a cluster, use the `kubectl` tool

### Control Plane

The main manager routines is in the 'Control Plane'.

This manages the worker nodes and pods in its cluster.

We usually run a control plane across multiple computers. Each hosting the same cluster config but with multiple different nodes. This provides high fault tolerance and availability.

### Components of a Node

The manager of a node is the kubelet. It makes sure that containers are running in a pod.

- A node may be a physical or virtual machine. It depends on the cluster
- A node must be at least running a kubelet and a container runtime (e.g. Docker). The container runtime pulls container images from a registry, unpacks it and runs the contained apps in pods

Note, a pod is just the set of running containers within the node.

So a kubelet takes a podspec and ensures that containers described in those specs are running and healthy.

Kublets dont manage containers that arent created by kubernetes.

- the kubelet registers its node with the api server (hostname, flags)

### Pods

A pod is an atomic unit of kubernetes. A pod may run 1 or more containers inside it.

A pod is always tied to a single node. It is scheduled there and remains there until termination. If a node failes, identical pods are scheduled on other available nodes in the cluster.

A pod may contain a bunch of containerised apps and volumes. Those apps should be using those volumes to do useful things like db access, reads, writes and stateful/stateless function routines.

- a pod is only accessible within the internal kubernetes cluster by default. Through its IPv4/6 addr. To make it accessible outside of the cluster, we have to expose the pod as a kubernetes service

### PodSpec?

Basically just a YAML/JSON object that describes a pod.

## Overview

Krustlet is a 'kubelet' aka node manager service. It runs inside a cluster defined by a kubernetes object spec, instead of the default kubelet program.

With krustlet we have a cool and simplified API for scheduling wasm-based containers in pods.
