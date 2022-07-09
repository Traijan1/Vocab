<template>
    <div class="home">
        <h2 :style="{ visibility }">{{translatedWord}}</h2>
        <div :style="{ visibility }">JLPT-Level: <span>{{jlptLevel}}</span></div>
        <input type="text" placeholder="Translate.." v-model="toTranslate" @keydown.enter="translateWord" /><br>
        <VocabButton @click="translateWord">Search</VocabButton>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import TranslateService from "../services/TranslateService";
import VocabButton from "../components/VocabButton.vue";
import { Visibility } from '../models/Visibility';

@Options({
    components: {
        VocabButton
    }
})

export default class JishoView extends Vue {
    toTranslate = "";
    translatedWord = ".";
    jlptLevel = ".";

    visibility = Visibility.Hidden;

    async translateWord() {
        [this.translatedWord, this.jlptLevel] = await TranslateService.getWord(this.toTranslate);

        this.translatedWord = this.translatedWord.toString().replaceAll(",", ", ");
        this.visibility = Visibility.Visible;
    }
}
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

.home {
    color: $primary-color;
    text-align: center;
    margin-top: 100px;

    h2 {
        margin: auto auto;
        margin-bottom: 5px;
        
        width: 90vw;
        font-size: 35px;
    }

    h2 + div {
        margin-bottom: 40px;
        font-size: 12px;

        span {
            text-transform: uppercase;
        }
    }
    
    input[type="text"] {
        @include vocab-input;
    }
}
</style>
