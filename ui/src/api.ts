import { AxiosResponse } from "axios";
import axios from "./http";
import { DateRangeRequest, LongResponser } from "./interfaces";

export function sunLong(
  data: DateRangeRequest
): Promise<AxiosResponse<Array<LongResponser>, any>> {
  return axios.post("/sun", data);
}

export function moonLong(
  data: DateRangeRequest
): Promise<AxiosResponse<Array<LongResponser>, any>> {
  return axios.post("/moon", data);
}

export function newMoonLong(
  data: DateRangeRequest
): Promise<AxiosResponse<Array<LongResponser>, any>> {
  return axios.post("/new_moon", data);
}
