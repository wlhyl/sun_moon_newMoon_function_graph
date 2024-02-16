<template>
  <div>
    <label>起始时间：</label>
    <input type="number" v-model="data.start.year" min="1900" max="2100" />
    <label>年</label>

    <input type="number" v-model="data.start.month" min="1" max="12" />
    <label>月</label>

    <input type="number" v-model="data.start.day" min="1" max="31" />
    <label>日</label>

    <input type="number" v-model="data.start.hour" min="0" max="23" />
    <label>时</label>

    <input type="number" v-model="data.start.minute" min="0" max="59" />
    <label>分</label>

    <input type="number" v-model="data.start.second" min="0" max="59" />
    <label>秒</label>

  </div>

  <div>
    <label>结束时间：</label>
    <input type="number" v-model="data.end.year" min="1900" max="2100" />
    <label>年</label>

    <input type="number" v-model="data.end.month" min="1" max="12" />
    <label>月</label>

    <input type="number" v-model="data.end.day" min="1" max="31" />
    <label>日</label>

    <input type="number" v-model="data.end.hour" min="0" max="23" />
    <label>时</label>

    <input type="number" v-model="data.end.minute" min="0" max="59" />
    <label>分</label>

    <input type="number" v-model="data.end.second" min="0" max="59" />
    <label>秒</label>

  </div>
  <div>
    <label>选择函数：</label>
    <select v-model="fun">
      <option :value="Function.Sun">太阳黄道经度</option>
      <option :value="Function.Moon">月亮黄道经度</option>
      <option :value="Function.NewMoon">新月黄道经度</option>
      <option :value="Function.Fun">f(x)=x^2</option>
    </select>
  </div>

  <button type="submit" @click="draw()">
    绘制
  </button>
  <div>
    <canvas id="canvas" width="0" height="0" style=" border: 2px solid black;"></canvas>
  </div>
</template>


<script setup lang="ts">
import { reactive, ref } from 'vue'
import { DateRangeRequest, LongResponser } from '../interfaces';
import { moonLong, newMoonLong, sunLong } from '../api';
import { AxiosResponse } from 'axios';
import { fabric } from 'fabric';

enum Function {
  Sun, Moon, NewMoon, Fun
}


const data = reactive(date())
const fun = ref(Function.Sun)

async function draw() {

  let f: (
    data: DateRangeRequest
  ) => Promise<AxiosResponse<Array<LongResponser>, any>>
  if (fun.value == Function.Sun) f = sunLong
  else if (fun.value == Function.Moon) f = moonLong
  else if (fun.value == Function.NewMoon) f = newMoonLong
  else f = fx

  const long = (await f(data)).data

  if (long.length < 2) return

  const canvas = new fabric.StaticCanvas('canvas');
  canvas.clear()

  // heigth: 360(角度最大值)+10(上边距)+10(下边距)
  // width: 365(日期最大值)+10(左边距)+10(右边距)
  const d = 10;// 橫坐标间隔
  canvas.setWidth(long.length * d + 20)
  canvas.setHeight(360 + 10 + 10)

  // 画x轴
  let x0 = 0
  let y0 = canvas.getHeight() - 10
  let x1 = canvas.getWidth()
  let y1 = y0
  let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
    stroke: 'black',
  });
  canvas.add(path);

  // 画y轴
  x0 = 10
  y0 = 0
  x1 = x0
  y1 = canvas.getHeight()
  path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
    stroke: 'black',
  });
  canvas.add(path);

  for (let i = 0; i < long.length - 1; i++) {

    if (long[i + 1].long < long[i].long) continue
    const x0 = (i + 1) * d + 10
    const y0 = 10 + 360 - long[i].long
    const x1 = x0 + d
    const y1 = 10 + 360 - long[i + 1].long

    const path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      stroke: 'black',
    });
    canvas.add(path);
  }

  // const zoom=2
  // canvas.setHeight(canvas.getHeight()*zoom)
  // canvas.setWidth(canvas.getWidth()*zoom)
  // canvas.setZoom(zoom);

}

function date(): DateRangeRequest {
  let t = new Date();
  let year = t.getFullYear();
  let month = t.getMonth() + 1;
  let day = t.getDate();
  let hour = t.getHours();
  let minute = t.getMinutes();
  let second = t.getSeconds();

  const start = { year, month, day, hour, minute, second }

  t.setFullYear(t.getFullYear() + 1)

  year = t.getFullYear();
  month = t.getMonth() + 1;
  day = t.getDate();
  hour = t.getHours();
  minute = t.getMinutes();
  second = t.getSeconds();

  const end = { year, month, day, hour, minute, second }

  return { start, end }

}

async function fx(_: DateRangeRequest): Promise<AxiosResponse<Array<LongResponser>, any>> {
  let x = [...Array(365).keys()].map((v, _) => {
    return {
      date: {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
      },
      long: v ** 2
    }
  })
  let y: AxiosResponse = {
    data: x,
    status: 0,
    statusText: '',
    headers: null,
    config: null
  }

  return y
}
</script>

<style scoped></style>
