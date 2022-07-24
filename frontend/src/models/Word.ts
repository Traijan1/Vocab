export class Word {
    public foreignLanguage: string;
    public motherTongue: string;
    public secondReading: string|undefined;

    constructor(foreignLanguage: string, motherTongue: string, secondReading: string|undefined) {
        this.foreignLanguage = foreignLanguage;
        this.motherTongue = motherTongue;
        this.secondReading = secondReading;
    }
}