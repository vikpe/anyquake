import { TaskOutcome, TaskStatus } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

export interface Command {
  name: string;
  module_id: string;
}

export class CommandTask {
  private readonly _command: Command;
  private _status: TaskStatus = TaskStatus.IDLE;
  private _outcome: TaskOutcome = TaskOutcome.UNDEFINED;
  private _data: string[] = [];
  private readonly _onChange: () => void;

  constructor(command: Command, onChange: () => void) {
    this._command = command;
    this._onChange = onChange;
  }

  get command() {
    return this._command;
  }

  get status() {
    return this._status;
  }

  get data() {
    return this._data;
  }

  get outcome() {
    return this._outcome;
  }

  async run() {
    if (this._status === TaskStatus.IN_PROGRESS) {
      throw new Error("Task is already in progress");
    } else if (this._status === TaskStatus.COMPLETED) {
      this._status = TaskStatus.IDLE;
      this._outcome = TaskOutcome.UNDEFINED;
    }

    this._status = TaskStatus.IN_PROGRESS;
    this._onChange();

    const result: string[] = await invoke("anyquake_command", {
      command: {
        [this._command.name]: {
          module_id: this._command.module_id,
        },
      },
    });
    this._data = result;
    this._status = TaskStatus.COMPLETED;
    this._outcome = result.length > 0 ? TaskOutcome.SUCCESS : TaskOutcome.FAIL;
    this._onChange();
  }
}

export class InvokeTask {
  private readonly _name: string;
  private _status: TaskStatus = TaskStatus.IDLE;
  private _outcome: TaskOutcome = TaskOutcome.UNDEFINED;
  private _data: string[] = [];
  private readonly _onChange: () => void;

  constructor(name: string, onChange: () => void) {
    this._name = name;
    this._onChange = onChange;
  }

  get name() {
    return this._name;
  }

  get status() {
    return this._status;
  }

  get data() {
    return this._data;
  }

  get outcome() {
    return this._outcome;
  }

  async run() {
    if (this._status === TaskStatus.IN_PROGRESS) {
      throw new Error("Task is already in progress");
    } else if (this._status === TaskStatus.COMPLETED) {
      this._status = TaskStatus.IDLE;
      this._outcome = TaskOutcome.UNDEFINED;
    }

    this._status = TaskStatus.IN_PROGRESS;
    this._onChange();

    const result: string[] = await invoke(this.name);
    this._data = result;
    this._status = TaskStatus.COMPLETED;
    this._outcome = result.length > 0 ? TaskOutcome.SUCCESS : TaskOutcome.FAIL;
    this._onChange();
  }
}
