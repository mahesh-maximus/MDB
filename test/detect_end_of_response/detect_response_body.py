import time

print ("<< Detect end of response >>");
read_buffer_size = 1024

def get_content_len(data):
    content_length_start_index = data.index("Content-Length: ") - 1
    content_length_end_index =  content_length_start_index + 17

    # print (content_length_start_index);
    # print (content_length_end_index);

    number_length = 1
    content_length = ""

    while(True):
        # time.sleep(2)
        temp = data[content_length_end_index:content_length_end_index + number_length]
        if temp.isnumeric():
            content_length = temp
            number_length += 1
            # print(content_length)
            # print(number_length)
        else:
            # print("break")
            break

    return int(content_length)

def get_content_start_index(data):
    response_body_seperator_srart_index = data.index("\\r\\n\\r\\n") - 1
    response_body_seperator_end_index = response_body_seperator_srart_index  + 9

    return response_body_seperator_end_index

    # print (response_body_seperator_end_index)

    # body = data[response_body_seperator_end_index:response_body_seperator_end_index + int(content_length)]

    print('Content Lenght:  {0} Body Length:  {1}'.format(content_length , len(body)))

    # return int(content_length) == len(body)

def get_response_len(data):
    content_len = get_content_len(data)
    content_start_index = get_content_start_index(data)
    response_len = content_start_index + content_len
    print (content_len)
    print (content_start_index)
    print (response_len)
    
    return response_len


print (">> Detect response len 144 ")
content_len =  0
content_read_len = 0
response_len = 0

while True:
    file = open("response_body_len_144.txt", "r")
    content_read_len = content_read_len + read_buffer_size
    temp_data = file.read(read_buffer_size)
    if response_len == 0:
        response_len = get_response_len(temp_data)
    if content_read_len >= response_len:
        print("End of content")
        break
    print("Not yet end of content");


print (">> Detect respone len 1025")
content_len =  0
content_read_len = 0
response_len = 0

while True:
    file = open("response_body_len_1025.txt", "r")
    content_read_len = content_read_len + read_buffer_size
    temp_data = file.read(read_buffer_size)
    if response_len == 0:
        response_len = get_response_len(temp_data)
    if content_read_len >= response_len:
        print("End of content")
        break
    print("Not yet end of content");


print (">> Detect respone len 1024")
content_len =  0
content_read_len = 0
response_len = 0

while True:
    file = open("response_body_len_1024.txt", "r")
    content_read_len = content_read_len + read_buffer_size
    temp_data = file.read(read_buffer_size)
    if response_len == 0:
        response_len = get_response_len(temp_data)
    if content_read_len >= response_len:
        print("End of content")
        break
    print("Not yet end of content");
