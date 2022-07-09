import { Word } from "@/models/Word";
import { Service } from "./Service";

class WordService extends Service {
    protected baseUrl = "http://192.168.178.98:8000";

    public async getAllWords(): Promise<Word[]> {
        const response = await fetch(`${this.baseUrl}/word`);
        const json = (await response.json()) as Word[];

        return json;
    }

    public async postWord(word: Word): Promise<void> {
        await fetch(`${this.baseUrl}/word`, {
            method: "POST",
            mode: "no-cors",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(word)
        });
    }
}

export default new WordService();
