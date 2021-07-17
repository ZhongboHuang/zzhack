use crate::article_service;
use crate::console_log;
use crate::pages::article::ArticleView;
use crate::services::article_service::Book;
use crate::services::article_service::Chapter;
use crate::services::article_service::User;
use crate::AppRoutes;
use crate::Article;
use css_in_rust::Style;
use material_yew::MatList;
use material_yew::MatListItem;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;

#[derive(Properties, Clone, Debug)]
pub struct BookViewProps {
    pub number: u32,
    pub article_number: Option<u32>,
    pub chapter_number: Option<u32>,
}

pub struct BookView {
    style: Style,
    book: Book,
    selected_chapter: Option<Chapter>,
    selected_article: Option<Article>,
    selected_content: String,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    link: ComponentLink<Self>,
    props: BookViewProps,
    user: User,
    selected_title: String,
}

pub enum BookViewMessage {
    ChangeRoute(AppRoutes),
    Nope,
}

impl Component for BookView {
    type Message = BookViewMessage;
    type Properties = BookViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "BookView",
            r#"
            display:flex;

            .side-bar {
                width: 300px;
                background: var(--side-bar-color);
                box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px, rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
            }
            
            .article-item {
                margin-left: 40px;
            }

            .article {
                flex: 1;
            }

            .list-item {

            }

            .side-bar-container {
                position: sticky;
                top: 48px;
            }

            @media (max-width: 600px){
                flex-direction: column;
                
                .side-bar {
                    width: 100%;
                    height: fit-content;
                }
            }
        "#,
        )
        .unwrap();
        let book = unsafe { article_service.get_book_by_number(2) };
        let route_agent = RouteAgent::bridge(link.callback(|_| BookViewMessage::Nope));
        let (selected_chapter, selected_article, selected_content, user, title) =
            parse_selected(props.chapter_number, props.article_number, &book);

        Self {
            style,
            book,
            selected_article: selected_article,
            selected_chapter,
            selected_content,
            selected_title: title,
            route_agent,
            link,
            props,
            user,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BookViewMessage::ChangeRoute(route) => {
                let book = &self.book;
                let parse_route =
                    move |chapter_number: Option<u32>, article_number: Option<u32>| {
                        parse_selected(chapter_number, article_number, book)
                    };

                let (selected_chapter, selected_article, selected_content, user, title) =
                    match route.clone() {
                        AppRoutes::Books(_) => parse_route(None, None),
                        AppRoutes::BooksWithArticle(_, chapter_number, article_number) => {
                            parse_route(Some(chapter_number), Some(article_number))
                        }
                        AppRoutes::BooksWithChapter(_, chapter_number) => {
                            parse_route(Some(chapter_number), None)
                        }
                        _ => parse_route(None, None),
                    };

                self.selected_article = selected_article;
                self.selected_chapter = selected_chapter;
                self.selected_content = selected_content.clone();
                self.user = user;
                self.route_agent.send(ChangeRoute(route.into()));
                self.selected_title = title;

                true
            }
            BookViewMessage::Nope => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let book_item_color = match &self.selected_chapter {
            Some(_) => "transparent",
            None => "rgba(57, 113, 245, 0.2)",
        };
        let match_article_item_color = |article: &Article| match &self.selected_article {
            Some(selected_article) => {
                if article.number == selected_article.number {
                    "rgba(57, 113, 245, 0.2)"
                } else {
                    "transparent"
                }
            }
            None => "transparent",
        };
        let match_chapter_item_color = |chapter: &Chapter| match &self.selected_chapter {
            Some(selected_chapter) => match &self.selected_article {
                Some(_) => "transparent",
                None => {
                    if chapter.number == selected_chapter.number {
                        "rgba(57, 113, 245, 0.2)"
                    } else {
                        "transparent"
                    }
                }
            },
            None => "transparent",
        };
        let get_active_list_item_style = |color: &'static str| {
            if color != "transparent" {
                format!(
                    "border-left: 4px solid var(--mdc-theme-primary);background: {};",
                    color
                )
            } else {
                format!("background: {};", color)
            }
        };
        let book_number = self.book.number;

        html! {
            <div class=self.style.to_string()>
                <div class="side-bar">
                    <div class="side-bar-container">
                        <MatList>
                            <div class="list-item" onclick=self.link.callback(move |_| BookViewMessage::ChangeRoute(AppRoutes::Books(book_number))) style=get_active_list_item_style(book_item_color)><MatListItem><span class="text">{self.book.title.clone()}</span></MatListItem></div>
                            {
                                for self.book.chapters.iter().map(|chapter| {
                                    let chapter_number = chapter.number;

                                    html! {
                                        <>
                                            <div class="list-item" onclick=self.link.callback(move |_| BookViewMessage::ChangeRoute(AppRoutes::BooksWithChapter(book_number,chapter_number))) style=get_active_list_item_style(match_chapter_item_color(chapter))>
                                                <MatListItem><span class="text">{chapter.title.clone()}</span></MatListItem>
                                            </div>
                                            {for chapter.articles.iter().map(|article| {
                                                let article_number = article.number;
                                                console_log!("{}",match_article_item_color(article));
                                                html! {
                                                    <div onclick=self.link.callback(move |_| BookViewMessage::ChangeRoute(AppRoutes::BooksWithArticle(book_number, chapter_number, article_number))) class="list-item" style=get_active_list_item_style(match_article_item_color(article))>
                                                        <MatListItem><span class="article-item text">{article.title.clone()}</span></MatListItem>
                                                    </div>
                                                }
                                            })}
                                        </>
                                    }
                                })
                            }
                        </MatList>
                    </div>
                </div>
                <div class="article">
                    <ArticleView title=self.selected_title.clone() user=self.user.clone() content=self.selected_content.clone() />
                </div>
            </div>
        }
    }
}

fn parse_selected(
    chapter_number: Option<u32>,
    article_number: Option<u32>,
    book: &Book,
) -> (Option<Chapter>, Option<Article>, String, User, String) {
    let mut selected_chapter: Option<Chapter> = None;
    let mut selected_article: Option<Article> = None;

    if let Some(chapter_number) = chapter_number {
        let chapter = book
            .chapters
            .iter()
            .find(|chapter| chapter.number == chapter_number)
            .unwrap()
            .clone();

        selected_chapter = Some(chapter.clone());

        if let Some(article_number) = article_number {
            selected_article = Some(
                chapter
                    .articles
                    .iter()
                    .find(|article| article.number == article_number)
                    .unwrap()
                    .clone(),
            );
        };
    };

    let cloned_book = book.clone();
    let (content, user, title) = match selected_chapter.clone() {
        None => (cloned_book.content, cloned_book.user, cloned_book.title),
        Some(chapter) => match selected_article.clone() {
            Some(article) => (article.body, article.user, article.title),
            None => (chapter.content, chapter.user, chapter.title),
        },
    };

    (selected_chapter, selected_article, content, user, title)
}