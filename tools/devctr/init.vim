:set number
call plug#begin()
Plug 'junegunn/vim-easy-align'
Plug 'airblade/vim-rooter'
Plug 'junegunn/fzf', { 'dir': '~/.fzf', 'do': './install --all' }
Plug 'junegunn/fzf.vim'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
call plug#end()

let g:coc_disable_startup_warning = 1

