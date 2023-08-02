export enum TaskStatus {
  IDLE = "IDLE",
  IN_PROGRESS = "IN_PROGRESS",
  COMPLETED = "COMPLETED",
}

export enum TaskOutcome {
  UNDEFINED = "UNDEFINED",
  SUCCESS = "SUCCESS",
  FAIL = "FAIL",
}

export interface TaskProps {
  name: string;
  status: TaskStatus;
  outcome: TaskOutcome;
  data: any;
}
