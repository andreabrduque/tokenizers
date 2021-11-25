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
        List<Long> ids = tokenizer.encode(tokenizeMe);
        List<String> list = new ArrayList<String>();
        list.add(tokenizeMe);
        list.add(tokenizeMe);
        List<Long> idsFromList = tokenizer.encode(list);
        System.out.println(String.format("ids from java str: %s", Arrays.toString(ids.toArray())));
        System.out.println(String.format("ids from java list: %s", Arrays.toString(idsFromList.toArray())));
    }
}