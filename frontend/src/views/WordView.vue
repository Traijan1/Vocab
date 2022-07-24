<template>
    <div class="view">
        <span v-if="currentWord != undefined">{{ currentWord.position }}/{{ words.length - 1 }} words done</span>

        <div>
            <h1 v-if="currentWord != undefined">{{ reading }}</h1>
            <div :style="{ visibility: hideSolution }" v-if="currentWord != undefined"><b>The word was: {{ currentWord.word.motherTongue }}</b></div>
            <input type="text" placeholder="Write translation here.." v-model="translation" @keydown.enter="checkWord" /> <br>
            <VocabButton @click="checkWord" class="button">Submit</VocabButton>
            <VocabButton @click="skip" class="button">Skip</VocabButton>
            <VocabButton @click="showSecondReading()" class="button" v-if="getHasSecondReading()">Second Reading</VocabButton>
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

    reading = "";
    translation = "";

    words: Word[] = [];

    currentWord: CurrentWord|undefined = undefined;

    hideSolution = Visibility.Hidden;

    async created() {
        this.words = await WordService.getAllWords();
        this.words = this.words.sort((_,b) => 0.5 - Math.random());
        this.words.push(new Word("You finished all words!", "", undefined));

        this.currentWord = {
            position: 0,
            word: this.words[0]
        };

        this.setReading();

        this.$forceUpdate();
        this.appStore.currentView = 1;
    }

    setReading() {
        if(this.currentWord == undefined)
            return;

        this.reading = this.currentWord.word.foreignLanguage;
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

        this.setReading();
        this.translation = "";
    }

    showSecondReading() {
        if(this.currentWord?.word.secondReading == undefined)
            return;

        this.reading = this.currentWord.word.secondReading;
    }

    getHasSecondReading(): boolean {
        return this.currentWord?.word.secondReading != undefined && this.currentWord?.word.secondReading != "";
    }
}
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

    .view {
        margin: 30px;
        color: white;

        div {
            margin-top: 25vh;
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
                margin-top: 20px;
            }

            input {
                @include vocab-input;
                margin: 0;
            }
        }
    }
</style>
