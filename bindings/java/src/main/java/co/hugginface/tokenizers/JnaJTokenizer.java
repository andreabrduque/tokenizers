package co.hugginface.tokenizers;

import com.sun.jna.*;
import com.sun.jna.ptr.PointerByReference;

import java.awt.*;
import java.lang.ref.Cleaner;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public interface JnaJTokenizer extends Library {

    JnaJTokenizer INSTANCE = (JnaJTokenizer) Native.load("tokenizers_jna", JnaJTokenizer.class);


    // to automatically free memory on the Rust side when GC'ed on JVM
    static final Cleaner cleaner = Cleaner.create();

    class JTokenizer extends PointerType {

        // according to https://techinplanet.com/java-9-cleaner-cleaner-cleanable-objects/,
        // it is wise to keep the cleaner runnables as a static class
        private static class CleanJTokenizer implements Runnable {
            private Pointer ptr;

            public CleanJTokenizer(Pointer ptr) {
                this.ptr = ptr;
            }

            @Override
            public void run() {
                JnaJTokenizer.INSTANCE.JTokenizer_drop(this.ptr);
            }
        }

        //check if it isnt null and create exception if it is
        public JTokenizer(String identifier) {
            Pointer pointer = INSTANCE.JTokenizer_from_pretrained(identifier);
            this.setPointer(pointer);
            cleaner.register(this, new CleanJTokenizer(pointer));
        }

        public void printTokenizer(){
            Pointer p = this.getPointer();
            INSTANCE.JTokenizer_print_tokenizer(p);
        }

        public List<Long> encode(String value){
            Pointer p = this.getPointer();
            Pointer pEncodings = INSTANCE.JTokenizer_encode_from_str(p, value);
            JEncoding encoding = new JEncoding(pEncodings);
            List<Long> ids = encoding.getIds();
            return ids;
        }

        //overloading with different types
        public List<Long> encode(List<String> values){
            StringArray sarray = new StringArray(values.toArray(new String[0]));
            PointerByReference parray = new PointerByReference();
            parray.setPointer(sarray);
            Pointer p = this.getPointer();
            Pointer pEncodings = INSTANCE.JTokenizer_encode_from_vec_str(p, parray, new size_t(values.size()));
            JEncoding encoding = new JEncoding(pEncodings);
            List<Long> ids = encoding.getIds();
            return ids;
        }
    }

    //the encoding IDS are unsigned, but I think this isnt java supported
    public static class size_t extends IntegerType {
        public size_t() { this(0); }
        public size_t(long value) { super(Native.SIZE_T_SIZE, value); }
    }

    class JEncoding extends PointerType {

        // according to https://techinplanet.com/java-9-cleaner-cleaner-cleanable-objects/,
        // it is wise to keep the cleaner runnables as a static class
        class CleanJEncoding implements Runnable {
            Pointer ptr;

            public CleanJEncoding(Pointer ptr) {
                this.ptr = ptr;
            }

            @Override
            public void run() {
                JnaJTokenizer.INSTANCE.JEncoding_drop(this.ptr);
            }
        }

        public JEncoding(Pointer initializer) {
            this.setPointer(initializer);
            JnaJTokenizer.cleaner.register(this, new CleanJEncoding(initializer));
        }

        public size_t getLength() {
            Pointer encodings = this.getPointer();
            size_t length = INSTANCE.JEncoding_get_length(encodings);
            return length;

        }
        public List<Long> getIds() {
            size_t idsSize = getLength();
            int isSizeInt = idsSize.intValue();
            Pointer buffer = new Memory((long) isSizeInt *Native.getNativeSize(long.class));
            Pointer encoding = this.getPointer();
            INSTANCE.JEncoding_get_ids(encoding, buffer, idsSize);
            long[] ids = buffer.getLongArray(0, isSizeInt);
            return Arrays.stream(ids).boxed().collect(Collectors.toList());
        }

//        public void printTokenizer(){
//            Pointer p = this.getPointer();
//            INSTANCE.JTokenizer_print_tokenizer(p);
//        }
//        //overloading with different types
//        public void encodeFromStr(String value){
//            Pointer p = this.getPointer();
//            INSTANCE.JTokenizer_encode_from_str(p, value);
//        }
    }


    //give separate types for the different pointers
    //the way it is now is very error prone
    Pointer JTokenizer_from_pretrained(String identifier);
    void JTokenizer_drop(Pointer tokenizer);
    Pointer JTokenizer_encode_from_str(Pointer tokenizer, String input);
    void JTokenizer_print_tokenizer(Pointer tokenizer);
    Pointer JTokenizer_encode_from_vec_str(Pointer tokenizer, PointerByReference parray, size_t sizeArray);
    void JEncoding_drop(Pointer tokenizer);
    size_t JEncoding_get_length(Pointer encoding);
    void JEncoding_get_ids(Pointer encoding, Pointer buffer, size_t sizeBuffer);


}
