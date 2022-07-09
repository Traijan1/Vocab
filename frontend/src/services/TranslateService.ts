import { Service } from "./Service";

class TranslateService extends Service {
    protected baseUrl = "http://192.168.178.98:8000/jisho";

    public async getWord(word: string): Promise<[string, string]> {
        const result = await fetch(`${this.baseUrl}?word=${word}`);

        if(result.status !== 200)
            return ["Jisho not available", ""];

        const json = JSON.parse(await result.json());
        console.log(json);

        if(json.data === undefined)
            return ["Word not found", ""];

        const data = json["data"][0];

        const [meaning, level] = [
            data["senses"][0]["english_definitions"], 
            (data["jlpt"].length > 0 ? data["jlpt"].toString() : "None").replace("jlpt-", "")
        ];

        return [meaning, level];
    }
}

export default new TranslateService();