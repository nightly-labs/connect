// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Device } from "./Device";
import type { Notification } from "./Notification";

export interface ConnectRequest { responseId: string, clientId: string, publicKeys: Array<string>, sessionId: string, notification?: Notification, device?: Device, metadata?: string, }