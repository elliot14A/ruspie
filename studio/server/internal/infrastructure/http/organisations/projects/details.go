package projects

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

func (h *httpHandler) details(w http.ResponseWriter, r *http.Request) {
	var err error
	var user_id uint = 1
	if helper.AuthEnable() {
		user_id, err = helper.GetUserID(r)
		if err != nil {
			h.logger.Error("error in parsing X-User header", "error", err.Error())
			errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User Header", http.StatusUnauthorized)))
			return
		}
	}

	p_id := helper.GetPathParamByName(r, "project_id")
	project_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing project_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id", http.StatusBadRequest)))
		return
	}
	o_id := helper.GetPathParamByName(r, "org_id")
	org_id, err := helper.StringToInt(o_id)
	if err != nil {
		h.logger.Error("error in parsing org_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid organisation_id", http.StatusBadRequest)))
		return
	}

	project, err := h.projectRepository.Details(uint(user_id), uint(org_id), uint(project_id))
	if err != nil {
		h.logger.Error("error in fetching project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in fetching project", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, project)
	return
}
