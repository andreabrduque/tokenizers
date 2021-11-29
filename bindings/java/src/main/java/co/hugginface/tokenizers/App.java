package co.hugginface.tokenizers;

import com.sun.jna.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class App {


    public static void main(String[] args) {
        String identifier = "bert-base-uncased";
        JnaJTokenizer.JTokenizer tokenizer = new JnaJTokenizer.JTokenizer(identifier);
        String tokenizeMe = "I love Java";
        JnaJTokenizer.JEncoding encodings = tokenizer.encode(tokenizeMe);
        List<String> list = new ArrayList<String>();
        list.add(tokenizeMe);
        list.add(tokenizeMe);
        JnaJTokenizer.JEncoding encodingsFromList = tokenizer.encode(list);
        //List<Integer> foo = tokenizer.encode_word_ids(tokenizeMe);
        System.out.println(String.format("ids from java str: %s", encodings.getIds()));
        System.out.println(String.format("ids from java list: %s", encodingsFromList.getIds()));
        System.out.println(String.format("tokens from java str: %s", encodings.getTokens()));
        System.out.println(String.format("tokens from java list: %s", encodingsFromList.getTokens()));

        //System.out.println(String.format("ids from java list: %s", Arrays.toString(foo.toArray())));


    }
}