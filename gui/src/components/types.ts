export enum TaskStatus {
    IDLE = "IDLE",
    IN_PROGRESS = "IN_PROGRESS",
    COMPLETED = "COMPLETED",
}

export enum TaskOutcome {
    UNDEFINED = "UNDEFINED",
    SUCCESS = "SUCCESS",
    FAIL = "FAIL"
}

export interface Task {
    status: TaskStatus;
    outcome: TaskOutcome;
    data: any;
}

export interface InstallationTask extends Task {
    data: QuakeInstallation[]
}

export interface QuakeInstallation {
    pak0_path: String,
    root_dir_path: String,
}
