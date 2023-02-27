export enum ResultType {
    Applications = 'Applications',
    Files = 'Files',
    Calculation = 'Calculation',
    Extensions = 'Extensions',
}

export interface InputResult {
    type: ResultType;
    title: string | null;
    value: string;
}
