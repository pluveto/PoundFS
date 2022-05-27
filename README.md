Install FUSE for Linux:

    sudo apt-get install fuse
    sudo apt-get install libfuse-dev pkg-config

Allocation Groups

一个磁盘被分为多个 AG

```
|<--------------------            XFS              ----------------------->|
+--------------+--------------+--------------+--------------+--------------+
|     AG-0     |     AG-1     |     AG-2     |     ....     |     AG-N     |
+--------------+--------------+--------------+--------------+--------------+
```

一个 AG 被分为以下部分

```
