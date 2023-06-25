export enum TaskStatus {
    IDLE,
    IN_PROGRESS,
    COMPLETED,
}

export enum TaskResult {
    UNDEFINED,
    SUCCESS,
    FAIL
}

export interface Task {
    status: TaskStatus;
    outcome: TaskResult;
    result: any;
}

export interface InstallationTask extends Task {
    result: QuakeInstallation
}

export interface QuakeInstallation {
    pak0_path: String,
    root_dir_path: String,
}
