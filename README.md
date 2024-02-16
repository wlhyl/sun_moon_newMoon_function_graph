** 绘制在太阳黄道经度、月亮黄道经度、新月黄道经度曲线 **

# 运行api
* 下载瑞士星历表，并编译
```bash
mkdir /tmp/swe
cd /tmp/swe
wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
tar xvzf swe.tar.gz
cd swisseph-2.10.03
make libswe.a
```

* 下载星历表文件
```bash
cd /tmp/swe
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semo_18.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semom48.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/sepl_18.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/seplm48.se1
```
* 运行api
```bash
cd api
ephe_path=/tmp/swe RUSTFLAGS=-L/tmp/swe/src cargo run  --features swagger,cors
```

* swagger，访问地址：
http://localhost:8080/swagger-ui/

# 运行ui
* 运行ui
运行ui需要Node.js，请先安装Node.js>=v18.17.1
```bash
cd ui
npm i
npm run dev
```

* ui 访问地址
http://localhost:5173
