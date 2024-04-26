package main


import (
	"fmt"
	"html/template"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"io"
    "net/http"
    "path/filepath"
    "strings"
    "log"
    "os"
)

type Template struct {
	templates *template.Template
}

func ParseTemplates() *template.Template {
    templ := template.New("")
    err := filepath.Walk("./src", func(path string, info os.FileInfo, err error) error {
        if strings.Contains(path, ".tmpl") {
            _, err = templ.ParseFiles(path)
            if err != nil {
                log.Println(err)
            }
        }

        return err
    })

    if err != nil {
        panic(err)
    }

    return templ
}

func StartServer(){
	t := &Template{
		templates: ParseTemplates(),
	}
	e := echo.New()
	e.Renderer = t

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
    e.Use(middleware.Static("lib"))
    e.Use(middleware.Static("assets"))

    e.GET("/", func(c echo.Context) error {
		return c.Render(http.StatusOK, "index" , nil)
    })
    e.GET("/playground", func(c echo.Context) error {
		return c.Render(http.StatusOK, "playground" , nil)
    })
    e.Logger.Fatal(e.Start(":1550"))
}


func (t *Template) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}

func main(){
    fmt.Println("Starting server!")
    StartServer()
}
