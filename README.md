## PoundFS

一个基于 Extent 和 B+tree 的 Unix 文件系统（开发中）。XFS 的子集实现。

### 开发环境

Install FUSE for Linux:

    sudo apt-get install fuse
    sudo apt-get install libfuse-dev pkg-config


### 介绍

Allocation Groups

一个磁盘被分为多个 AG

```
|<--------------------            XFS              ----------------------->|
+--------------+--------------+--------------+--------------+--------------+
|     AG-0     |     AG-1     |     AG-2     |     ....     |     AG-N     |
+--------------+--------------+--------------+--------------+--------------+
```
