<template>
    <div class="view">
        <span>{{ currentWord.position }}/{{ words.length }} words left</span>

        <div>
            <h1>{{ currentWord.word.foreignLanguage }}</h1>
            <div :style="{ visibility: hideSolution }"><b>The word was: {{ currentWord.word.motherTongue }}</b></div>
            <input type="text" placeholder="Write translation here.." v-model="translation" @keydown.enter="checkWord" /> <br>
            <VocabButton @click="checkWord">Submit</VocabButton>
            <VocabButton @click="skip">Skip</VocabButton>
        </div>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { Word } from "../models/Word";
import VocabButton from "../components/VocabButton.vue";

@Options({
    props: {
        language: String,
        category: String,
    },
    components: {
        VocabButton
    }
})
export default class WordView extends Vue {
    language!: string;
    category!: string;

    translation = "";

    words: Word[] =  [
        new Word("日本", "Japan"),
        new Word("買う", "kaufen"),
        new Word("好き", "mögen"),
        new Word("私", "Ich"),
        new Word("彼女", "Sie"),
        new Word("何", "Was"),
    ];

    currentWord = {
        position: 0,
        word: this.words[0]
    };

    hideSolution = "hidden";

    created() {
        this.words = this.words.sort((a,b) => 0.5 - Math.random());
    }

    checkWord() {
        if(this.currentWord.word.motherTongue == this.translation)
            this.nextWord();
    }

    skip() {
        this.hideSolution = "visible";
    }

    nextWord() {
        this.hideSolution = "hidden";
        if(this.currentWord.position == this.words.length)
            return;

        const current = this.currentWord;
        this.currentWord = {
            position: current.position += 1,
            word: this.words[current.position]
        };
        this.translation = "";
    }
}
</script>

<style lang="scss" scoped>
    .view {
        margin: 30px;
        color: white;

        div {
            margin-top: 200px;
            text-align: center;

            h1, div {
                color: #FFF8C9;
            } 

            h1 {
                font-size: 40px;
                margin-bottom: 20px;
            }

            div {
                font-size: 14px;
                margin: 0;
                margin-bottom: 30px;
            }

            input {
                padding: 10px;
                border-radius: 10px;
                border: none;
                width: 60vw;
                background-color: #343434;
                color: white;
                margin-bottom: 20px;
                
                &::placeholder {
                    color: white;
                }
            }
        }
    }
</style>
