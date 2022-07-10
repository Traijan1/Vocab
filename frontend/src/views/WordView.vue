<template>
    <div class="view">
        <span v-if="currentWord != undefined">{{ currentWord.position }}/{{ words.length - 1 }} words left</span>

        <div>
            <h1 v-if="currentWord != undefined">{{ currentWord.word.foreignLanguage }}</h1>
            <div :style="{ visibility: hideSolution }" v-if="currentWord != undefined"><b>The word was: {{ currentWord.word.motherTongue }}</b></div>
            <input type="text" placeholder="Write translation here.." v-model="translation" @keydown.enter="checkWord" /> <br>
            <VocabButton @click="checkWord" class="button">Submit</VocabButton>
            <VocabButton @click="skip" class="button">Skip</VocabButton>
        </div>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { Word } from "../models/Word";
import VocabButton from "../components/VocabButton.vue";
import WordService from "../services/WordService";
import { CurrentWord } from '../models/CurrentWord';
import { Visibility } from '@/models/Visibility';
import { useAppStore } from '@/stores/AppStore';

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
    appStore = useAppStore();

    language!: string;
    category!: string;

    translation = "";

    words: Word[] =  [
        // new Word("日本", "Japan"),
        // new Word("買う", "kaufen"),
        // new Word("好き", "mögen"),
        // new Word("私", "Ich"),
        // new Word("彼女", "Sie"),
        // new Word("何", "Was"),
    ];

    currentWord: CurrentWord|undefined = undefined;

    hideSolution = Visibility.Hidden;

    async created() {
        this.words = await WordService.getAllWords();
        this.words = this.words.sort((_,b) => 0.5 - Math.random());
        this.words.push(new Word("You finished all words!", ""));

        this.currentWord = {
            position: 0,
            word: this.words[0]
        };

        this.$forceUpdate();
        this.appStore.currentView = 1;
    }

    checkWord() {
        if(this.currentWord === undefined)
            return;

        if(this.currentWord.word.motherTongue == this.translation)
            this.nextWord();
    }

    skip() {
        this.hideSolution = Visibility.Visible;
    }

    nextWord() {
        if(this.currentWord === undefined)
            return;

        this.hideSolution = Visibility.Hidden;

        if(this.currentWord.position > this.words.length)
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
@import "@/assets/variables.scss";

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

            .button {
                margin: 0 15px;
            }

            input {
                @include vocab-input;
            }
        }
    }
</style>
