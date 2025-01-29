# bmpuzzle
Embed files within the padding of bitmap images.

## Usage
Insert hidden.jpeg into original_image.bmp
```bash
bmpuzzle -i original_image.bmp hidden.jpeg
```

Extract extracted_data.jpeg out of image_with_hidden_data.bmp
```bash
bmpuzzle -e image_with_hidden_data.bmp extracted_data.jpeg
```
