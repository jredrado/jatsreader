#-----------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See LICENSE in the project root for license information.
#-----------------------------------------------------------------------------------------

from flask import Flask
from flask import jsonify

import requests 
import tempfile

import ebooklib
from ebooklib import epub
from ebooklib.utils import debug
from ebooklib.utils import parse_html_string
from ebooklib.plugins.base import BasePlugin

from bs4 import BeautifulSoup
from bs4 import Tag
from bs4 import NavigableString

import spacy

from spacy_langdetect import LanguageDetector
from spacy.lang.en import English

from spacy.language import Language
import chardet


WINDOWS_LINE_ENDING = '\r\n' #CRLF
UNIX_LINE_ENDING = '\n' #LF

@Language.factory("language_detector")
def get_lang_detector(nlp, name):
   return LanguageDetector()

nlp = English()
nlp.add_pipe('sentencizer')
nlp.add_pipe('language_detector', last=True)

#import nltk
#nltk.download('punkt')

from  htmltokenizer import HTMLSentenceTokenizer

import networkx as nx

class SentencizerPlugin(BasePlugin):
    NAME = 'Sentencizer Plugin'

    NEW_HTML = """<html><head>
                        <title>The Dormouse's story</title>
                        <style type="text/css">
                            @namespace l "http://linguisticannotation.com";
                            l|sentence {  border:1px solid red;\n cursor:default;\n box-sizing:border-box;}
                        </style>
                        </head>
                <body></body>"""

    def html_before_write(self, book, chapter):
        if chapter.content is None:
            return
        
        print(chapter)

        encoding = chardet.detect(chapter.content)['encoding']

        soup,st,result,sentences = html_with_sentences(chapter.content.decode(encoding) )

        chapter.content = soup.prettify(encoding)




def generator_strings(l,plain=True):
    N = len(l)
    for i in range(N):
        M = len(l[i])
        for j in range(M):
            if plain:
                yield(l[i][j])
            else:
                yield (i,j,l[i][j])

def search(pat, txt):
    lx = [c for c in generator_strings(txt)]

    M = len(pat)
    N = len(lx)

    for i in range(N-M+1):
        status = 1 
        for j in range(M):
            if lx[i+j][2] != pat[j]:
                status = 0
                break
        if j == M-1 and status != 0:
            return (lx[i][0],lx[i][1],lx[i+M-1][0],lx[i+M-1][1])
    return None

def search2(pat, lx):
    
    M = len(pat)
    N = len(lx)

    for i in range(N-M+1):
        status = 1 
        for j in range(M):
            if lx[i+j] != pat[j]:
                status = 0
                break
        if j == M-1 and status != 0:
            return (i,lx[i],i+M-1,lx[i+M-1])
    return None

def search_sentences (txt,sentences,mark=0):
    lx = "".join(generator_strings(txt))
    lx2 = [c for c in generator_strings(txt,False)]

    result = []
    start_index = 0
    for s in sentences:
        M = len(s)
        try:
            result_index  = lx.index(s,start_index)
            result.append ((lx2[result_index],lx2[result_index + M-1]))
            start_index = result_index + M

        except ValueError:
            print(mark, " search_sentences: ValueError ", start_index,s)
            pass

    return result

def download_url(url, save_path, chunk_size=128):
    r = requests.get(url, stream=True)
    with open(save_path, 'wb') as fd:
        for chunk in r.iter_content(chunk_size=chunk_size):
            fd.write(chunk)

def download_url_to_tempfile(url, chunk_size=128):
    r = requests.get(url, stream=True)
    fd = tempfile.NamedTemporaryFile()
    for chunk in r.iter_content(chunk_size=chunk_size):
        fd.write(chunk)
    return fd

app = Flask(__name__)


structural_items = ('section',)
text_items = ('title','p', 'div', 'h1', 'h2', 'h3', 'h4')

def create_sentences (text):
        #Creating sentences

        sentences = []
        doc = nlp(text)
        for sent in doc.sents:
            node = dict()
            node['text'] = sent.text
            node['language'] = sent._.language['language']
            words = []
            for word in sent:
                words.append(word.text)
            node['words'] = words

            sentences.append(node)

        return sentences

def walker(element,accumulator):

    current_acc = accumulator

    if element.name in structural_items:
        node = dict(name=element.name)
        if 'childrens' not in accumulator:
            accumulator['childrens'] = []
        accumulator['childrens'].append(node)
        current_acc = node
    elif element.name in text_items:
        node = dict(name=element.name)
        if 'childrens' not in accumulator:
            accumulator['childrens'] = []
        accumulator['childrens'].append(node)   
        node['sentences'] = create_sentences(element.text)

        current_acc = node

    if hasattr(element, 'children'):
        for child in element.children:
            walker(child,current_acc)

def get_tags_strings(body):
    st = []
    #tags = []

    for t in body.strings:
        st.append(t)
        #tags.append(t.parent)
    return st

def get_epub_info (path):
    book = epub.read_epub(path)

    new_html = """<html><head>
                        <title>The Dormouse's story</title>
                        <style type="text/css">
                            @namespace l "http://linguisticannotation.com";
                            l|sentence {  border:1px solid red;\n cursor:default;\n box-sizing:border-box;}
                        </style>
                        </head>
                <body></body>"""

    new_soup = BeautifulSoup(new_html,'lxml')

    result = []

    i = 0
    for item in  book.get_items_of_type(ebooklib.ITEM_DOCUMENT):

        encoding = chardet.detect(item.content)['encoding']

        soup,st,result,sentences = html_with_sentences(item.content.decode(encoding) )
        #print (soup)
        with open("output_" + str(i) , "w") as file:
            file.write(soup.prettify())

        i +=1
        result.extend(soup.body.contents)

    #Get Dublin core metadata
    #try:
    #    result['metadata'] = book.metadata['http://purl.org/dc/elements/1.1/']
    #except KeyError:
    #    pass

    new_soup.html['xmlns:l']='http://linguisticannotation.com'
    
    new_soup.body.contents = result

    #print(new_soup)
    return new_soup

def html_to_graph(soup):
    G = nx.Graph()

    for node in soup.descendants:
        G.add_node(node)

        if (node.parent):
            #Parent route is allowed but siblings are prefered
            G.add_edge(node.parent,node,weight=1000)
        if (node.next_sibling):
            G.add_edge(node,node.next_sibling,weight=0)
        if (node.previous_sibling):
            G.add_edge(node,node.previous_sibling,weight=0)

    return G

def get_split_strings (result,st):
    split_candidates = {}
    for (init_index,i,_),(end_index,l,_) in result:
                if len(st[end_index]) != l+1:
                    if not st[end_index] in split_candidates:
                        split_candidates[st[end_index]]={'last_index':-1,'split_list':[]}
                    if (init_index == end_index):
                        split_candidates[st[end_index]]['split_list'].append((i,l+1))
                    else:
                        split_candidates[st[end_index]]['split_list'].append((0,l+1))
                    split_candidates[st[end_index]]['last_index']=l+1
    return split_candidates

def split_sentences (split_candidates):
    
    for s in split_candidates:
        split_list = split_candidates[s]['split_list']
        last_index = split_candidates[s]['last_index']

        splited_strings = [NavigableString(s[i:e]) for (i,e) in split_list]
        splited_strings.append(NavigableString(s[last_index:]))
        #Modifiy html tree
        if splited_strings:
            parent = s.parent
            s.replace_with(splited_strings[0])
            #Get position of new node and insert after that
            index = parent.index(splited_strings[0])
            for i in range(1,len(splited_strings)):
                try:
                    parent.insert(index + i,splited_strings[i])
                except ValueError:
                    print ("ValueError ",parent,index + i,splited_strings[i])


def html_with_sentences(html_doc):
    #html_doc = """ <div> Test infun<p>da<b>do</b>.<span>Esto es una</span><span>nueva prueba.</span>Otro texto</p></div>"""
    #html_doc = ' <div>Una primera frase. Una segunda. Test infunda<b>do</b>.<p><span>Esto es una</span><span>nueva prueba. Otra frase difícil.</span> Otro texto</p></div>'

    normalized_html = html_doc.replace(WINDOWS_LINE_ENDING,UNIX_LINE_ENDING)

    soup = BeautifulSoup(normalized_html, 'html.parser')
    
    st = get_tags_strings(soup.body)

    sentences = HTMLSentenceTokenizer().feed(normalized_html)
    result = search_sentences(st,sentences,0)

    #Split sentences
    split_candidates = get_split_strings(result,st)
    #Modify html soup
    split_sentences(split_candidates)

    #Reframe result
    #sentences = HTMLSentenceTokenizer().feed(str(soup))
    st = get_tags_strings(soup.body)
    result = search_sentences(st,sentences,1)
    G = html_to_graph(soup)

    #Wrap sentences
    merge_element_list = []
    for (init_index,_,_),(end_index,_,_) in result:

        shortest_path = nx.shortest_path(G,st[init_index],st[end_index],'weight')
        #Remove duplicate/child elements
        shortest_path_tags = [tag for tag in shortest_path if isinstance(tag,Tag)]
    
        merge_elements = []
        for e in shortest_path:
            remove = False
            for c in shortest_path_tags:
                if e in c.descendants:
                    remove=True
                    break
            if not remove:
                merge_elements.append(e)

        if (merge_elements):
            merge_element_list.append(merge_elements)
    
    #soup.html['xmlns:l']='http://linguisticannotation.com'

    #head = soup.head
    #head.append(soup.new_tag('style', type='text/css'))
    #head.style.append('@namespace l "http://linguisticannotation.com";\n l|sentence {  border:1px solid red;\n cursor:default;\n box-sizing:border-box;}')

    for merge_elements in merge_element_list:

        initial_element = merge_elements[0]

        sentence_tag = soup.new_tag('sentence')
        initial_element.wrap(sentence_tag)

        for i in range(1,len(merge_elements)):
            sentence_tag.append(merge_elements[i])
  
 
                

    return (soup,st,result,sentences)

def test():
    main_html = open('../../epubcontract/assets/childrens-literature/EPUB/s04.xhtml', 'r').read()
    soup,st,result,sentences = html_with_sentences(main_html)
    with open("output1.html", "w") as file:
        file.write(soup.prettify())
    return soup,st,result,sentences

def test2():
    fd = download_url_to_tempfile('https://github.com/IDPF/epub3-samples/releases/download/20170606/childrens-literature.epub')
    result = get_epub_info(fd)
    return str(result)

def test3():
    path = download_url_to_tempfile('https://github.com/IDPF/epub3-samples/releases/download/20170606/childrens-literature.epub')
    book = epub.read_epub(path)
    opts = {'plugins': [SentencizerPlugin()]}

    # create epub file
    epub.write_epub('test.epub', book, opts)

@app.route("/")
def hello():
    #fd = download_url_to_tempfile('https://www.gutenberg.org/ebooks/98.epub.images')
    fd = download_url_to_tempfile('https://github.com/IDPF/epub3-samples/releases/download/20170606/childrens-literature.epub')
    result = get_epub_info(fd)
    return str(result)


if __name__ == "__main__":
    import argparse

    msg = "EPub sentence tokenizer"
 
    # Initialize parser
    parser = argparse.ArgumentParser(description = msg)
    parser.add_argument("-i", "--Input", help = "Input epub file")
    parser.add_argument("-o", "--Output", help = "Output epub file")
 
    # Read arguments from command line
    args = parser.parse_args()
 
    if args.Output and args.Input:
        print("Displaying Input as: % s" % args.Input)
        print("Displaying Output as: % s" % args.Output)

        book = epub.read_epub(args.Input)
        opts = {'plugins': [SentencizerPlugin()]}

        # create epub file
        epub.write_epub(args.Output, book, opts)

